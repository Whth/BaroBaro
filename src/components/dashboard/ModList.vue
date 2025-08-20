<template>
  <div class="mod-list">
    <h2>Installed Mods</h2>
    <div class="mod-list-container" @dragover="handleDragOver">
      <ModCard
        v-for="(mod, index) in installed_mod"
        :key="mod.steamWorkshopId"
        :mod="mod"
        :index="index"
        :is-enabled="isModEnabled(mod.steamWorkshopId, currentModList)"
        :draggable="true"
        :is-drag-over="dragOverIndex === index"
        @toggle-mod="toggleMod"
        @select-mod="selectMod"
        @dragstart="handleDragStart(index, $event)"
        @dragover="handleDragOver"
        @dragenter="handleDragEnter(index)"
        @dragleave="handleDragLeave"
        @drop="handleDrop(index, $event)"
        @dragend="handleDragEnd"
      />
    </div>
    <div class="mod-order-actions">
      <button class="save-order-button" @click="saveModOrder">Save Mod Order</button>
      <button class="load-order-button" @click="loadModOrder">Load Mod Order</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ModCard from './ModCard.vue'
import { installed_mod, mod_lists, isModEnabled } from '../../composables/useModManager'

// Drag and drop state
const draggedItemIndex = ref<number | null>(null)
const dragOverIndex = ref<number | null>(null)

// For now, we'll use the first mod list as the current one
// In a real implementation, this would be managed by a global state or prop
const currentModList = mod_lists.value.length > 0 ? mod_lists.value[0] : null

const toggleMod = (modId: string) => {
  // In a real implementation, this would call a Tauri command to toggle the mod
  console.log(`Toggle mod ${modId}`)
}

const selectMod = (modId: string) => {
  console.log(`Mod ${modId} selected`)
  // In a real app, this would emit an event to show mod details
}

// Drag and drop methods
const handleDragStart = (index: number, event: DragEvent) => {
  draggedItemIndex.value = index
  // Add visual feedback
  if (event.dataTransfer) {
    event.dataTransfer.setData('text/plain', index.toString())
    event.dataTransfer.effectAllowed = 'move'
  }
}

const handleDragOver = (event: DragEvent) => {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
  return false
}

const handleDragEnter = (index: number) => {
  dragOverIndex.value = index
}

const handleDragLeave = () => {
  dragOverIndex.value = null
}

const handleDrop = (index: number, event: DragEvent) => {
  event.preventDefault()
  
  if (draggedItemIndex.value === null) return
  
  // In a real implementation, this would reorder the mods in the current profile
  console.log(`Dropped mod at index ${index}`)
  
  // Reset drag state
  draggedItemIndex.value = null
  dragOverIndex.value = null
}

const handleDragEnd = () => {
  draggedItemIndex.value = null
  dragOverIndex.value = null
}

// Save and load mod order as XML
const saveModOrder = () => {
  // In a real implementation, this would save the current mod order to a profile
  console.log('Saving mod order')
  alert('Saving mod order - this would connect to the Rust backend in a real implementation')
}

const loadModOrder = () => {
  // In a real app, this would load mod order from a profile
  console.log('Loading mod order')
  alert('Loading mod order - this would connect to the Rust backend in a real implementation')
}
</script>

<style scoped>
.mod-list {
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
  padding: var(--spacing-l);
}

.mod-list h2 {
  margin-top: 0;
  margin-bottom: var(--spacing-m);
}

.mod-list-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.mod-order-actions {
  display: flex;
  gap: var(--spacing-m);
  margin-top: var(--spacing-m);
}

.save-order-button, .load-order-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.save-order-button {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}
</style>