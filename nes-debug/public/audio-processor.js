// AudioWorkletProcessor 用于处理音频流
class AudioProcessor extends AudioWorkletProcessor {
    constructor() {
        super()
        console.log('[AudioProcessor] 初始化')

        // 音频队列
        this.audioQueue = []

        // 当前播放缓冲区
        this.currentBuffer = null
        this.currentBufferIndex = 0

        // 是否已经开始播放
        this.isPlaying = false

        // 监听来自主线程的消息
        this.port.onmessage = event => {
            const { type, data } = event.data

            switch (type) {
                case 'audio-data':

                    // 将音频数据加入队列
                    const buffer = new Float32Array(data)
                    this.audioQueue.push(buffer)

                    // 只要有数据就开始播放（低延迟模式）
                    if (!this.isPlaying && this.audioQueue.length > 0) {
                        this.isPlaying = true
                    }
                    break
                case 'clear':

                    // 清空音频队列
                    this.audioQueue = []
                    this.currentBuffer = null
                    this.currentBufferIndex = 0
                    this.isPlaying = false
                    console.log('[AudioProcessor] 队列已清空')
                    break
            }
        }
    }

    process(inputs, outputs) {

        // 获取输出缓冲区
        const output = outputs[0]
        if (!output || output.length === 0) {
            return true
        }

        const channel = output[0]
        const length = channel.length

        // 计算队列中剩余的样本数
        let queuedSamples = 0
        if (this.currentBuffer) {
            queuedSamples += this.currentBuffer.length - this.currentBufferIndex
        }
        for (const buf of this.audioQueue) {
            queuedSamples += buf.length
        }

        // 如果缓冲不足，输出静音
        if (!this.isPlaying) {
            for (let i = 0; i < length; i++) {
                channel[i] = 0
            }

            return true
        }

        // 每 100 次输出一次队列状态
        if (!this.logCounter) this.logCounter = 0
        this.logCounter++
        if (this.logCounter % 100 === 0) {
            console.log(`[AudioProcessor] 队列样本: ${queuedSamples}, 队列数: ${this.audioQueue.length}`)
        }

        // 填充输出缓冲区
        for (let i = 0; i < length; i++) {
            if (this.currentBuffer && this.currentBufferIndex < this.currentBuffer.length) {

                // 从当前缓冲区读取
                channel[i] = this.currentBuffer[this.currentBufferIndex]
                this.currentBufferIndex++
            }
            else {

                // 当前缓冲区已用完，尝试获取下一个
                this.currentBuffer = null
                this.currentBufferIndex = 0

                if (this.audioQueue.length > 0) {
                    this.currentBuffer = this.audioQueue.shift()
                    channel[i] = this.currentBuffer[this.currentBufferIndex]
                    this.currentBufferIndex++
                }
                else {

                    // 没有更多音频数据，停止播放并填充静音
                    if (this.isPlaying) {
                        console.log('[AudioProcessor] 队列空，停止播放')
                    }
                    this.isPlaying = false
                    channel[i] = 0
                }
            }
        }

        return true
    }
}

// 注册处理器
registerProcessor('audio-processor', AudioProcessor)
