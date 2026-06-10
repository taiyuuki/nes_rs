<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { NametableData } from '../types'

const props = defineProps<{
    running: boolean;
    tick:    number;
}>()

const canvas0 = ref<HTMLCanvasElement | null>(null)
const canvas1 = ref<HTMLCanvasElement | null>(null)
const canvas2 = ref<HTMLCanvasElement | null>(null)
const canvas3 = ref<HTMLCanvasElement | null>(null)

const selectedTable = ref(0)
const enabled = ref(false)

// 从 localStorage 读取状态
onMounted(() => {
    const saved = localStorage.getItem('nametable-enabled')
    if (saved !== null) {
        enabled.value = saved === 'true'
    }
})

// 监听状态变化并保存
watch(enabled, value => {
    localStorage.setItem('nametable-enabled', String(value))
    if (value && props.running) {
        fetchData()
    }
})

async function fetchData() {
    if (!props.running || !enabled.value) return
    try {
        const data = await invoke<NametableData>('get_nametables')

        // 只渲染当前选中的 nametable
        switch (selectedTable.value) {
            case 0:
                renderTable(canvas0.value, data.table0, data.width, data.height)
                break
            case 1:
                renderTable(canvas1.value, data.table1, data.width, data.height)
                break
            case 2:
                renderTable(canvas2.value, data.table2, data.width, data.height)
                break
            case 3:
                renderTable(canvas3.value, data.table3, data.width, data.height)
                break
        }
    }
    catch {

        // ignore
    }
}

function renderTable(canvas: HTMLCanvasElement | null, pixels: number[], width: number, height: number) {
    if (!canvas || pixels.length === 0) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return
    const imageData = ctx.createImageData(width, height)
    imageData.data.set(pixels)
    ctx.putImageData(imageData, 0, 0)
}

watch(() => props.tick, fetchData)
watch(() => props.running, r => { if (r && enabled.value) fetchData() })
watch(selectedTable, () => { if (enabled.value) fetchData() })
onMounted(() => { if (props.running && enabled.value) fetchData() })

const tableNames = ['$2000', '$2400', '$2800', '$2C00']
</script>

<template>
  <div class="panel">
    <div class="flex items-center justify-between mb-1.5">
      <h3 class="panel-title mb-0">
        Nametables
      </h3>
      <button
        :class="['toggle-btn', enabled ? 'toggle-btn-on' : 'toggle-btn-off']"
        title="启用/禁用 Nametable 渲染"
        @click="enabled = !enabled"
      >
        {{ enabled ? 'ON' : 'OFF' }}
      </button>
    </div>

    <div
      v-show="enabled"
      class="table-selector mb-2"
    >
      <button
        v-for="(name, idx) in tableNames"
        :key="idx"
        :class="['table-btn', selectedTable === idx ? 'table-btn-active' : '']"
        @click="selectedTable = idx"
      >
        {{ name }}
      </button>
    </div>

    <div
      v-show="enabled"
      class="nametable-container"
    >
      <canvas
        v-show="selectedTable === 0"
        ref="canvas0"
        :width="256"
        :height="240"
        class="nametable-canvas"
      />
      <canvas
        v-show="selectedTable === 1"
        ref="canvas1"
        :width="256"
        :height="240"
        class="nametable-canvas"
      />
      <canvas
        v-show="selectedTable === 2"
        ref="canvas2"
        :width="256"
        :height="240"
        class="nametable-canvas"
      />
      <canvas
        v-show="selectedTable === 3"
        ref="canvas3"
        :width="256"
        :height="240"
        class="nametable-canvas"
      />
    </div>

    <div
      v-show="!enabled"
      class="text-[#888] text-xs py-2 text-center"
    >
      Nametable 渲染已禁用以提升性能
    </div>
  </div>
</template>

<style scoped>
@reference "tailwindcss";
.panel {
  @apply bg-[#16213e] rounded p-2 border border-[#0f3460];
}
.panel-title {
  @apply text-xs font-bold text-[#4fc3f7] mb-1.5 uppercase tracking-wider;
}
.toggle-btn {
  @apply text-[10px] font-bold rounded py-0.5 px-2 transition-colors;
}
.toggle-btn-on {
  @apply text-[#4fc3f7] bg-[#0f3460];
}
.toggle-btn-off {
  @apply text-[#666] bg-[#1a1a2e] hover:bg-[#252540];
}
.table-selector {
  @apply flex gap-1;
}
.table-btn {
  @apply flex-1 text-[10px] text-[#888] bg-[#1a1a2e] rounded py-1 px-2 hover:bg-[#0f3460] hover:text-[#e0e0e0] transition-colors;
}
.table-btn-active {
  @apply text-[#4fc3f7] bg-[#0f3460];
}
.nametable-container {
  @apply relative;
}
.nametable-canvas {
  image-rendering: pixelated;
  image-rendering: crisp-edges;
  width: 100%;
  height: auto;
  background: #181818;
  border: 1px solid #0f3460;
  border-radius: 2px;
}
</style>
