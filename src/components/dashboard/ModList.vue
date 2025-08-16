<template>
  <div class="mod-list">
    <h2>Installed Mods</h2>
    <div class="mod-list-container">
      <ModCard
        v-for="mod in mods"
        :key="mod.id"
        :mod="mod"
        @toggle-mod="toggleMod"
        @select-mod="selectMod"
      />
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
</style>