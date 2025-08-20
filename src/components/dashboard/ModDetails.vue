<template>
  <div class="mod-details">
    <h2>Mod Details</h2>
    <div v-if="selectedMod" class="mod-details-content">
      <div class="mod-header">
        <h3>{{ selectedMod.name }}</h3>
        <button
          class="toggle-button"
          @click="toggleMod"
        >
          Toggle
        </button>
      </div>
      <div class="mod-info-grid">
        <div class="mod-info-item">
          <span class="info-label">Version:</span>
          <span class="info-value">{{ selectedMod.modVersion }}</span>
        </div>
        <div class="mod-info-item">
          <span class="info-label">Steam ID:</span>
          <span class="info-value">{{ selectedMod.steamWorkshopId }}</span>
        </div>
        <div class="mod-info-item">
          <span class="info-label">Type:</span>
          <span class="info-value" :class="`source-${selectedMod.corePackage ? 'core' : 'mod'}`">
            {{ selectedMod.corePackage ? 'Core Package' : 'Mod' }}
          </span>
        </div>
        <div class="mod-info-item">
          <span class="info-label">Game Version:</span>
          <span class="info-value">{{ selectedMod.gameVersion }}</span>
        </div>
      </div>
      <div class="mod-description">
        <h4>File Groups</h4>
        <p>{{ Object.keys(selectedMod.fileGroups).length }} file groups</p>
      </div>
      <div class="mod-actions">
        <button class="action-button update-button">Update</button>
        <button class="action-button remove-button">Remove</button>
      </div>
    </div>
    <div v-else class="no-selection">
      <p>Select a mod to view details</p>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { BarotraumaMod } from '../../proto/mods'

const selectedMod = ref<BarotraumaMod | null>(null)

// In a real implementation, this would be set by the parent component or a global state
// For now, we'll set it to the first installed mod if available
onMounted(() => {
  // This would be set by the parent component in a real implementation
  console.log('ModDetails mounted')
})

const toggleMod = () => {
  if (selectedMod.value) {
    // In a real implementation, this would call a Tauri command to toggle the mod
    console.log(`Toggle mod ${selectedMod.value.name}`)
  }
}

// Since BarotraumaMod doesn't have date fields, we'll return a placeholder
const formatDate = (date: Date) => {
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}
</script>

<style scoped>
.mod-details {
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
  padding: var(--spacing-l);
}

.mod-details h2 {
  margin-top: 0;
  margin-bottom: var(--spacing-m);
}

.mod-details-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.mod-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.mod-header h3 {
  margin: 0;
  font-size: var(--font-size-heading-2);
  color: var(--color-text-primary);
}

.toggle-button {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-size: var(--font-size-body-small);
  font-weight: var(--font-weight-medium);
}

.toggle-enabled {
  background-color: var(--color-success);
  color: white;
  border-color: var(--color-success);
}

.mod-info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: var(--spacing-m);
}

.mod-info-item {
  display: flex;
  flex-direction: column;
}

.info-label {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
  font-weight: var(--font-weight-medium);
}

.info-value {
  font-size: var(--font-size-body-regular);
  color: var(--color-text-primary);
}

.source-local {
  color: var(--color-primary);
  font-weight: var(--font-weight-bold);
}

.source-remote {
  color: var(--color-info);
  font-weight: var(--font-weight-bold);
}

.mod-description h4 {
  margin: 0 0 var(--spacing-s) 0;
  color: var(--color-text-primary);
}

.mod-description p {
  margin: 0;
  color: var(--color-text-primary);
  line-height: 1.5;
}

.mod-actions {
  display: flex;
  gap: var(--spacing-m);
  padding-top: var(--spacing-m);
  border-top: 1px solid var(--color-border);
}

.action-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.update-button {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.remove-button {
  background-color: var(--color-error);
  color: white;
  border-color: var(--color-error);
}

.no-selection {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-secondary);
}
</style>