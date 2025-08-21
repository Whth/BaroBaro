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
          :key="mod.steamWorkshopId"
          class="mod-remove-card"
          :class="{ 'selected': selectedMods.includes(mod.steamWorkshopId) }"
        >
          <div class="mod-info">
            <input
              type="checkbox"
              :id="`mod-${mod.steamWorkshopId}`"
              :value="mod.steamWorkshopId"
              v-model="selectedMods"
              class="mod-checkbox"
            />
            <label :for="`mod-${mod.steamWorkshopId}`" class="mod-label">
              <h3 class="mod-name">{{ mod.name }}</h3>
              <p class="mod-version">Version: {{ mod.modVersion }}</p>
              <p class="mod-description">Steam ID: {{ mod.steamWorkshopId }}</p>
            </label>
          </div>
          <div class="mod-actions">
            <button
              class="remove-button"
              @click="removeMod(mod.steamWorkshopId)"
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
import { installed_mod, refreshInstalledMods } from '../../composables/useModManager'

const searchQuery = ref('')

const selectedMods = ref<string[]>([])

const filteredMods = computed(() => {
  if (!searchQuery.value) {
    return installed_mod.value
  }
  const query = searchQuery.value.toLowerCase()
  return installed_mod.value.filter(mod =>
    mod.name.toLowerCase().includes(query) ||
    mod.steamWorkshopId.toLowerCase().includes(query)
  )
})

const removeMod = async (modId: string) => {
  const mod = installed_mod.value.find(m => m.steamWorkshopId === modId)
  if (mod) {
    if (confirm(`Are you sure you want to remove "${mod.name}"?`)) {
      // In a real app, this would call the Tauri backend to remove the mod
      console.log(`Removing mod: ${mod.name}`)
      try {
        // For now, we'll just simulate the removal and refresh the installed mods list
        alert(`Removed ${mod.name} successfully!`)
        // Refresh the installed mods list after removal
        await refreshInstalledMods()
      } catch (error) {
        console.error('Failed to remove mod:', error)
        alert(`Failed to remove ${mod.name}.`)
      }
    }
  }
}

const removeSelectedMods = async () => {
  if (selectedMods.value.length === 0) return
  
  if (confirm(`Are you sure you want to remove ${selectedMods.value.length} mod(s)?`)) {
    // In a real app, this would call the Tauri backend to remove the mods
    console.log(`Removing ${selectedMods.value.length} mods`)
    try {
      // For now, we'll just simulate the removal and refresh the installed mods list
      alert(`Removed ${selectedMods.value.length} mod(s) successfully!`)
      // Refresh the installed mods list after removal
      await refreshInstalledMods()
      // Clear selection
      selectedMods.value = []
    } catch (error) {
      console.error('Failed to remove mods:', error)
      alert('Failed to remove mods.')
    }
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