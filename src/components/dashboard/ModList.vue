<template>
  <div class="mod-list">
    <h2>Installed Mods</h2>
    <div class="mod-list-container" @dragover="handleDragOver">
      <ModCard
        v-for="(mod, index) in mods"
        :key="mod.id"
        :mod="mod"
        :index="index"
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
import { ref, onMounted } from 'vue'
import ModCard from './ModCard.vue'

interface Mod {
  id: string
  name: string
  version: string
  author: string
  description: string
  enabled: boolean
  installedAt: Date
  updatedAt: Date
  dependencies: string[]
  conflicts: string[]
  filePath: string
  source: 'local' | 'remote'
}

const mods = ref<Mod[]>([
  {
    id: '1',
    name: 'Better Graphics',
    version: '1.2.3',
    author: 'GraphicsMaster',
    description: 'Enhances the game graphics with better textures and lighting',
    enabled: true,
    installedAt: new Date('2025-01-15'),
    updatedAt: new Date('2025-01-15'),
    dependencies: [],
    conflicts: [],
    filePath: '/mods/better-graphics',
    source: 'local'
  },
  {
    id: '2',
    name: 'New Weapons Pack',
    version: '2.0.1',
    author: 'WeaponSmith',
    description: 'Adds 50+ new weapons to the game',
    enabled: false,
    installedAt: new Date('2025-02-20'),
    updatedAt: new Date('2025-02-20'),
    dependencies: [],
    conflicts: [],
    filePath: '/mods/new-weapons',
    source: 'remote'
  },
  {
    id: '3',
    name: 'Improved AI',
    version: '1.5.0',
    author: 'AIExpert',
    description: 'Makes NPCs more intelligent and responsive',
    enabled: true,
    installedAt: new Date('2025-03-10'),
    updatedAt: new Date('2025-03-10'),
    dependencies: [],
    conflicts: ['2'],
    filePath: '/mods/improved-ai',
    source: 'remote'
  }
])

// Drag and drop state
const draggedItemIndex = ref<number | null>(null)
const dragOverIndex = ref<number | null>(null)

const toggleMod = (modId: string) => {
  const mod = mods.value.find(m => m.id === modId)
  if (mod) {
    mod.enabled = !mod.enabled
    console.log(`Mod ${mod.name} toggled to ${mod.enabled ? 'enabled' : 'disabled'}`)
  }
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
  
  // Reorder the mods array
  const draggedItem = mods.value[draggedItemIndex.value]
  mods.value.splice(draggedItemIndex.value, 1)
  mods.value.splice(index, 0, draggedItem)
  
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
  // Create XML structure
  let xml = '<?xml version="1.0" encoding="UTF-8"?>\n'
  xml += '<modOrder>\n'
  
  mods.value.forEach((mod, index) => {
    xml += `  <mod priority="${index}" id="${mod.id}" name="${mod.name}" />\n`
  })
  
  xml += '</modOrder>'
  
  // In a real app, this would be sent to the backend
  console.log('Saving mod order as XML:', xml)
  
  // For demo purposes, we'll just show an alert
  alert('Mod order saved as XML!\n\n' + xml)
}

const loadModOrder = () => {
  // In a real app, this would load from the backend
  console.log('Loading mod order from XML')
  alert('Loading mod order from XML - this would connect to the Rust backend in a real implementation')
}

onMounted(() => {
  console.log('Mod list mounted with', mods.value.length, 'mods')
})
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