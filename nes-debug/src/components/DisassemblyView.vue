<script setup lang="ts">
import type { DisasmResult } from '../types'

defineProps<{ result: DisasmResult | null; }>()

function hexAddr(a: number): string {
    return a.toString(16).toUpperCase()
        .padStart(4, '0')
}

function hexBytes(bytes: [number, number, number], len: number): string {
    const parts: string[] = []
    for (let i = 0; i < len; i++) {
        parts.push(bytes[i].toString(16).toUpperCase()
            .padStart(2, '0'))
    }

    return parts.join(' ').padEnd(8)
}
</script>

<template>
  <div class="panel">
    <h3 class="panel-title">
      Disassembly
    </h3>
    <div
      v-if="result"
      class="font-mono text-[10px] leading-[16px] overflow-auto max-h-[280px]"
    >
      <div
        v-for="(inst, idx) in result.instructions"
        :key="inst.address"
        :class="idx === result.pc_index ? 'pc-row' : 'row'"
      >
        <span class="arrow">{{ idx === result.pc_index ? ">" : " " }}</span>
        <span class="addr">${{ hexAddr(inst.address) }}</span>
        <span class="hex">{{ hexBytes(inst.bytes, inst.len) }}</span>
        <span class="mnemonic">{{ inst.mnemonic }}</span>
        <span class="operand">{{ inst.operand }}</span>
      </div>
    </div>
    <div
      v-else
      class="text-[#666] text-xs"
    >
      No data
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
.pc-row {
  @apply flex bg-[#0f3460] text-[#4fc3f7];
}
.row {
  @apply flex text-[#aaa];
}
.arrow {
  @apply w-3 shrink-0 text-center;
}
.addr {
  @apply w-[42px] shrink-0;
}
.hex {
  @apply w-[56px] shrink-0 mr-2 text-[#666];
}
.mnemonic {
  @apply w-9 shrink-0 text-[#e0e0e0];
}
.operand {
  @apply text-[#81d4fa];
}
</style>
