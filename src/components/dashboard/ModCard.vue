<template>
  <n-card :segmented="{ content: true }">
    <n-thing>
      <template #header>
        <div style="display: flex; align-items: center; gap: 12px; flex-wrap: wrap;">
          <n-text strong style="font-size: 16px;">{{ mod.name }}</n-text>
          <n-tag v-if="mod.corePackage" size="small" type="info">Core Package</n-tag>
          <n-tag v-if="mod.homeDir" size="small" type="success">Local Mod</n-tag>
        </div>
      </template>

      <template #description>
        <n-space :size="6" style="margin-top: 8px;" vertical>
          <n-descriptions :column="1" label-placement="left" size="small">
            <n-descriptions-item label="Mod Version">
              <n-text code>{{ mod.modVersion }}</n-text>
            </n-descriptions-item>
            <n-descriptions-item label="Game Version">
              <n-text code>{{ mod.gameVersion }}</n-text>
            </n-descriptions-item>
            <n-descriptions-item label="Steam Workshop ID">
              <n-text code>{{ mod.steamWorkshopId }}</n-text>
            </n-descriptions-item>
            <n-descriptions-item label="Expected Hash">
              <n-text code style="word-break: break-all;">{{ mod.expectedHash }}</n-text>
            </n-descriptions-item>
            <n-descriptions-item v-if="mod.homeDir" label="Home Directory">
              <n-text code style="word-break: break-all;">{{ mod.homeDir }}</n-text>
            </n-descriptions-item>
          </n-descriptions>
        </n-space>
      </template>
    </n-thing>
  </n-card>
</template>

<script lang="ts" setup>
import type {BarotraumaMod} from '../../proto/mods.ts'

interface Props {
  mod: BarotraumaMod
}

defineProps<Props>()
</script>