<template>
  <div class="remove-mod-tab">
    <h2>Remove Mods</h2>
    <div class="remove-content">
      <div class="search-section">
        <div class="search-container">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search mods to remove..."
            class="search-input"
          />
          <button class="search-button">🔍</button>
        </div>
      </div>
      <div class="installed-mods">
        <div v-if="filteredMods.length === 0" class="no-mods">
          No mods found
        </div>
        <div
          v-else
          v-for="mod in filteredMods"
          :key="mod.id"
          class="mod-remove-card"
          :class="{ 'selected': selectedMods.includes(mod.id) }"
        >
          <div class="mod-info">
            <input
              type="checkbox"
              :id="`mod-${mod.id}`"
              :value="mod.id"
              v-model="selectedMods"
              class="mod-checkbox"
            />
            <label :for="`mod-${mod.id}`" class="mod-label">
              <h3 class="mod-name">{{ mod.name }}</h3>
              <p class="mod-version">Version: {{ mod.version }}</p>
              <p class="mod-description">{{ mod.description }}</p>
            </label>
          </div>
          <div class="mod-actions">
            <button
              class="remove-button"
              @click="removeMod(mod.id)"
            >
              Remove
            </button>
          </div>
        </div>
      </div>
      <div v-if="selectedMods.length > 0" class="bulk-actions">
        <p>{{ selectedMods.length }} mod(s) selected</p>
        <button class="remove-selected-button" @click="removeSelectedMods">
          Remove Selected
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Mod {
  id: string
  name: string
  version: string
  description: string
}

const searchQuery = ref('')

const mods = ref<Mod[]>([
  {
    id: '1',
    name: 'Better Graphics',
    version: '1.3.0',
    description: 'Enhanced textures and lighting improvements'
  },
  {
    id: '2',
    name: 'New Weapons Pack',
    version: '2.1.0',
    description: 'Added 10 new weapons and balanced existing ones'
  },
  {
    id: '3',
    name: 'Improved AI',
    version: '1.5.0',
    description: 'Makes NPCs more intelligent and responsive'
  },
  {
    id: '4',
    name: 'Custom Maps',
    version: '1.0.2',
    description: 'Adds 5 custom maps to the game'
  }
])

const selectedMods = ref<string[]>([])

const filteredMods = computed(() => {
  if (!searchQuery.value) {
    return mods.value
  }
  const query = searchQuery.value.toLowerCase()
  return mods.value.filter(mod =>
    mod.name.toLowerCase().includes(query) ||
    mod.description.toLowerCase().includes(query)
  )
})

const removeMod = (modId: string) => {
  const mod = mods.value.find(m => m.id === modId)
  if (mod) {
    if (confirm(`Are you sure you want to remove "${mod.name}"?`)) {
      // Remove from mods list
      mods.value = mods.value.filter(m => m.id !== modId)
      // Remove from selected if it was selected
      selectedMods.value = selectedMods.value.filter(id => id !== modId)
      console.log(`Removed mod: ${mod.name}`)
      alert(`Removed ${mod.name} successfully!`)
    }
  }
}

const removeSelectedMods = () => {
  if (selectedMods.value.length === 0) return
  
  if (confirm(`Are you sure you want to remove ${selectedMods.value.length} mod(s)?`)) {
    // Remove selected mods
    mods.value = mods.value.filter(mod => !selectedMods.value.includes(mod.id))
    console.log(`Removed ${selectedMods.value.length} mods`)
    alert(`Removed ${selectedMods.value.length} mod(s) successfully!`)
    // Clear selection
    selectedMods.value = []
  }
}
</script>

<style scoped>
.remove-mod-tab h2 {
  margin-top: 0;
  margin-bottom: var(--spacing-l);
}

.remove-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-l);
}

.search-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.search-container {
  display: flex;
  align-items: center;
  max-width: 500px;
}

.search-input {
  flex: 1;
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.search-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-left: none;
  border-radius: var(--border-radius-soft);
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
}

.installed-mods {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.no-mods {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-secondary);
}

.mod-remove-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  background-color: var(--color-surface);
  transition: all 0.2s ease;
}

.mod-remove-card:hover {
  border-color: var(--color-primary-light);
}

.mod-remove-card.selected {
  border-color: var(--color-primary);
  background-color: rgba(59, 130, 246, 0.05);
}

.mod-info {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-m);
  flex: 1;
}

.mod-checkbox {
  margin-top: var(--spacing-xs);
}

.mod-label {
  display: flex;
  flex-direction: column;
  cursor: pointer;
}

.mod-name {
  margin: 0 0 var(--spacing-xs) 0;
  font-size: var(--font-size-heading-3);
  color: var(--color-text-primary);
}

.mod-version {
  margin: 0 0 var(--spacing-xs) 0;
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.mod-description {
  margin: 0;
  color: var(--color-text-primary);
  font-size: var(--font-size-body-small);
  line-height: 1.4;
}

.mod-actions {
  display: flex;
  gap: var(--spacing-s);
}

.remove-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-error);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.bulk-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  background-color: var(--color-surface);
}

.bulk-actions p {
  margin: 0;
  color: var(--color-text-primary);
  font-weight: var(--font-weight-medium);
}

.remove-selected-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-error);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}
</style>