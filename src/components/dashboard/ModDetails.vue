<template>
  <n-card :bordered="false" class="mod-details-card">
    <h2>Mod Details</h2>

    <div v-if="selectedMod" class="mod-content">
      <!-- Header -->
      <div class="mod-header">
        <n-h3>{{ selectedMod.name }}</n-h3>
        <n-button
            secondary
            strong
            type="primary"
            @click="toggleMod"
        >
          Toggle
        </n-button>
      </div>

      <!-- Info Grid -->
      <n-grid :cols="2" :x-gap="12" :y-gap="12">
        <n-gi>
          <n-statistic :value="selectedMod.modVersion" label="Version"/>
        </n-gi>
        <n-gi>
          <n-statistic :value="selectedMod.steamWorkshopId" label="Steam ID"/>
        </n-gi>
        <n-gi>
          <n-statistic
              :class="selectedMod.corePackage ? 'core-package' : 'regular-mod'"
              :value="selectedMod.corePackage ? 'Core Package' : 'Mod'"
              label="Type"
          />
        </n-gi>
        <n-gi>
          <n-statistic :value="selectedMod.gameVersion" label="Game Version"/>
        </n-gi>
      </n-grid>

      <!-- File Groups -->
      <div class="mod-description">
        <n-h4>File Groups</n-h4>
        <p>{{ Object.keys(selectedMod.fileGroups).length }} file groups</p>
      </div>

      <!-- Actions -->
      <div class="mod-actions">
        <n-button type="primary" @click="updateMod">Update</n-button>
        <n-button type="error" @click="removeMod">Remove</n-button>
      </div>
    </div>

    <n-empty v-else description="Select a mod to view details"/>
  </n-card>
</template>

<script lang="ts" setup>
import type {BarotraumaMod} from "../../proto/mods"

// Props (接收父组件传入的选中mod)
const props = defineProps<{
  selectedMod: BarotraumaMod | null
}>()

// Emits (向父组件发送事件)
const emit = defineEmits<{
  (e: 'toggle', mod: BarotraumaMod): void
  (e: 'update', mod: BarotraumaMod): void
  (e: 'remove', mod: BarotraumaMod): void
}>()

// 操作函数
const toggleMod = () => {
  if (props.selectedMod) {
    emit('toggle', props.selectedMod)
  }
}

const updateMod = () => {
  if (props.selectedMod) {
    emit('update', props.selectedMod)
  }
}

const removeMod = () => {
  if (props.selectedMod) {
    emit('remove', props.selectedMod)
  }
}
</script>

<style scoped>
.mod-details-card {
  height: 100%;
}

.mod-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.mod-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.mod-description {
  margin: 10px 0;
}

.mod-description p {
  color: #666;
}

.mod-actions {
  display: flex;
  gap: 12px;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid #eee;
}

.core-package {
  color: #18a058;
}

.regular-mod {
  color: #f0a020;
}
</style>