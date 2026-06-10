import { onMounted, onUnmounted, ref, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { AudioData } from '../types'

// AudioWorklet 处理器 URL（public 目录中的静态资源）
const workletUrl = '/audio-processor.js'

export function useAudioPlayer() {
    const audioContext = ref<AudioContext | null>(null)
    const workletNode = ref<AudioWorkletNode | null>(null)
    const volume = ref(0.5)
    const enabled = ref(false)
    const isPlaying = ref(false)

    let unlisten: (() => void) | null = null
    let gainNode: GainNode | null = null
    let audioBuffer: Float32Array | null = null
    let accumulatedSamples = 0
    const BUFFER_THRESHOLD = 1470 // 约 33ms 的缓冲

    // 从 localStorage 读取状态
    onMounted(() => {
        const saved = localStorage.getItem('audio-enabled')
        if (saved === 'true') {
            enabled.value = true
        }
        const savedVolume = localStorage.getItem('audio-volume')
        if (savedVolume) {
            volume.value = Number.parseFloat(savedVolume)
        }

        // 初始化 AudioContext（如果启用）
        if (enabled.value) {
            initAudioContext()
        }

        // 监听后端音频事件
        setupAudioListener()
    })

    // 初始化 AudioContext
    async function initAudioContext() {
        if (!audioContext.value) {

            // 不强制设置采样率，使用系统默认值
            audioContext.value = new AudioContext()
            console.log(`[Audio] AudioContext 创建，采样率: ${audioContext.value.sampleRate}Hz`)
        }

        // 恢复 AudioContext（浏览器可能暂停它）
        if (audioContext.value.state === 'suspended') {
            await audioContext.value.resume()
        }

        // 加载 AudioWorklet module
        if (!workletNode.value) {
            try {
                await audioContext.value.audioWorklet.addModule(workletUrl)
                console.log('[AudioWorklet] Module 加载成功')

                // 创建 AudioWorkletNode，设置更大的 buffer size 以降低消费频率
                workletNode.value = new AudioWorkletNode(
                    audioContext.value,
                    'audio-processor',
                    {
                        processorOptions: {

                            // 设置为 4096 样本，约 93ms，减少 process 调用频率
                            bufferSize: 4096,
                        },
                    },
                )

                // 创建增益节点用于音量控制
                gainNode = audioContext.value.createGain()
                gainNode.gain.value = volume.value

                // 连接节点：worklet -> gain -> destination
                workletNode.value.connect(gainNode)
                gainNode.connect(audioContext.value.destination)

                console.log('[AudioWorklet] 音频节点已连接')

                // 通知后端使用正确的采样率
                console.log(`[Audio] 需要后端采样率: ${audioContext.value.sampleRate}Hz`)
            }
            catch(error) {
                console.error('[AudioWorklet] 初始化失败:', error)
            }
        }
    }

    // 重采样函数：将音频从源采样率转换到目标采样率
    function resampleAudio(samples: Float32Array, fromRate: number, toRate: number): Float32Array {
        if (fromRate === toRate) {
            return samples
        }

        const ratio = fromRate / toRate
        const outputLength = Math.round(samples.length / ratio)
        const output = new Float32Array(outputLength)

        for (let i = 0; i < outputLength; i++) {
            const srcIndex = i * ratio
            const srcIndexLow = Math.floor(srcIndex)
            const srcIndexHigh = Math.min(srcIndexLow + 1, samples.length - 1)
            const frac = srcIndex - srcIndexLow

            // 线性插值
            output[i] = samples[srcIndexLow] * (1 - frac) + samples[srcIndexHigh] * frac
        }

        console.log(`[Audio] 重采样: ${fromRate}Hz -> ${toRate}Hz, 样本: ${samples.length} -> ${outputLength}`)

        return output
    }

    // 播放音频样本
    function playAudioSamples(audioData: AudioData) {
        if (!enabled.value || !workletNode.value || audioData.samples.length === 0) {
            return
        }

        let samples = new Float32Array(audioData.samples)
        const targetRate = audioContext.value?.sampleRate || 44100

        // 如果采样率不匹配，进行重采样
        if (audioData.sample_rate && audioData.sample_rate !== targetRate) {
            samples = resampleAudio(samples, audioData.sample_rate, targetRate) as Float32Array<ArrayBuffer>
        }

        // 累积音频数据
        if (!audioBuffer || audioBuffer.length - accumulatedSamples < samples.length) {

            // 需要扩展或创建新缓冲区
            const newBuffer = new Float32Array((audioBuffer?.length || 0) + samples.length + BUFFER_THRESHOLD)
            if (audioBuffer) {
                newBuffer.set(audioBuffer, 0)
            }
            audioBuffer = newBuffer
        }

        // 添加新样本到缓冲区
        audioBuffer!.set(samples, accumulatedSamples)
        accumulatedSamples += samples.length

        // 当累积足够数据时，发送到 AudioWorklet
        if (accumulatedSamples >= BUFFER_THRESHOLD) {
            const samplesToSend = audioBuffer!.subarray(0, accumulatedSamples)

            console.log(`[Audio] 发送 ${accumulatedSamples} 样本到 AudioWorklet`)

            workletNode.value.port.postMessage({
                type: 'audio-data',
                data: samplesToSend.buffer,
            }, [samplesToSend.buffer])

            // 重置缓冲区（保留剩余部分）
            const remaining = audioBuffer!.length - accumulatedSamples
            if (remaining > 0) {
                audioBuffer = audioBuffer!.subarray(accumulatedSamples)

                // 创建新副本避免使用 subarray
                const newBuffer = new Float32Array(remaining)
                newBuffer.set(audioBuffer)
                audioBuffer = newBuffer
            }
            else {
                audioBuffer = null
            }
            accumulatedSamples = 0

            isPlaying.value = true
        }

        // 更新增益
        if (gainNode) {
            gainNode.gain.value = volume.value
        }
    }

    // 设置音频事件监听
    async function setupAudioListener() {
        if (unlisten) {
            unlisten()
        }

        try {
            unlisten = await listen<AudioData>('audio_data', event => {
                if (enabled.value) {
                    playAudioSamples(event.payload)
                }
            })
            console.log('[AudioWorklet] 音频事件监听已设置')
        }
        catch(error) {
            console.error('[AudioWorklet] 设置音频监听失败:', error)
        }
    }

    // 移除音频事件监听
    function removeAudioListener() {
        if (unlisten) {
            unlisten()
            unlisten = null
        }
    }

    // 启用音频
    async function enable() {
        await initAudioContext()
        enabled.value = true
    }

    // 禁用音频
    function disable() {
        enabled.value = false
        removeAudioListener()

        // 清空 AudioWorklet 队列
        if (workletNode.value) {
            workletNode.value.port.postMessage({ type: 'clear' })
        }

        // 断开连接
        if (workletNode.value) {
            workletNode.value.disconnect()
            workletNode.value = null
        }

        if (audioContext.value) {
            audioContext.value.close()
            audioContext.value = null
        }

        // 清空缓冲区
        audioBuffer = null
        accumulatedSamples = 0
        isPlaying.value = false
        gainNode = null
    }

    // 设置音量
    function setVolume(value: number) {
        volume.value = Math.max(0, Math.min(1, value))
    }

    // 切换启用状态
    function toggle() {
        if (enabled.value) {
            disable()
        }
        else {
            enable()
        }
    }

    // 监听状态变化
    watch(enabled, value => {
        localStorage.setItem('audio-enabled', String(value))
        if (value) {
            setupAudioListener()
        }
        else {
            removeAudioListener()
        }
    })

    watch(volume, value => {
        localStorage.setItem('audio-volume', String(value))
    })

    // 清理
    onUnmounted(() => {
        disable()
        removeAudioListener()
    })

    return {
        enabled,
        volume,
        isPlaying,
        enable,
        disable,
        toggle,
        setVolume,
    }
}
