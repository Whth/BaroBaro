<template>
  <div class="ui-preferences">
    <h2>UI Preferences</h2>
    <div class="settings-form">
      <div class="form-group">
        <label for="accent-color" class="form-label">Accent Color</label>
        <div class="color-picker">
          <input
            id="accent-color"
            v-model="preferences.accentColor"
            type="color"
            class="color-input"
          />
          <span class="color-value">{{ preferences.accentColor }}</span>
        </div>
      </div>
      <div class="form-group">
        <label for="layout-density" class="form-label">Layout Density</label>
        <select id="layout-density" v-model="preferences.layoutDensity" class="form-select">
          <option value="compact">Compact</option>
          <option value="normal">Normal</option>
          <option value="spacious">Spacious</option>
        </select>
      </div>
      <div class="form-group">
        <label class="form-label">Sidebar Position</label>
        <div class="radio-group">
          <label class="radio-option">
            <input
              v-model="preferences.sidebarPosition"
              type="radio"
              value="left"
              class="form-radio"
            />
            Left
          </label>
          <label class="radio-option">
            <input
              v-model="preferences.sidebarPosition"
              type="radio"
              value="right"
              class="form-radio"
            />
            Right
          </label>
        </div>
      </div>
      <div class="form-group">
        <label for="animation-speed" class="form-label">Animation Speed</label>
        <input
          id="animation-speed"
          v-model.number="preferences.animationSpeed"
          type="range"
          min="0"
          max="2"
          step="0.1"
          class="form-range"
        />
        <div class="range-labels">
          <span>Slow</span>
          <span>Normal</span>
          <span>Fast</span>
        </div>
      </div>
      <div class="form-group">
        <label for="show-tooltips" class="form-label">
          <input
            id="show-tooltips"
            v-model="preferences.showTooltips"
            type="checkbox"
            class="form-checkbox"
          />
          Show tooltips
        </label>
      </div>
      <div class="form-actions">
        <button class="save-button" @click="savePreferences">Save Preferences</button>
        <button class="reset-button" @click="resetPreferences">Reset to Defaults</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface UIPreferences {
  accentColor: string
  layoutDensity: string
  sidebarPosition: string
  animationSpeed: number
  showTooltips: boolean
}

const preferences = ref<UIPreferences>({
  accentColor: '#3B82F6',
  layoutDensity: 'normal',
  sidebarPosition: 'left',
  animationSpeed: 1,
  showTooltips: true
})

const savePreferences = () => {
  console.log('Saving UI preferences:', preferences.value)
  // In a real app, this would save to the backend or local storage
  alert('UI preferences saved successfully!')
}

const resetPreferences = () => {
  if (confirm('Are you sure you want to reset all UI preferences to defaults?')) {
    preferences.value = {
      accentColor: '#3B82F6',
      layoutDensity: 'normal',
      sidebarPosition: 'left',
      animationSpeed: 1,
      showTooltips: true
    }
    console.log('UI preferences reset to defaults')
  }
}

onMounted(() => {
  console.log('UI preferences mounted')
})
</script>

<style scoped>
.ui-preferences h2 {
  margin: 0 0 var(--spacing-l) 0;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-label {
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.color-picker {
  display: flex;
  align-items: center;
  gap: var(--spacing-m);
}

.color-input {
  width: 50px;
  height: 30px;
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
}

.color-value {
  font-family: var(--font-family-monospace);
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.form-select {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
  width: 100%;
  max-width: 300px;
}

.radio-group {
  display: flex;
  gap: var(--spacing-m);
}

.radio-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-weight: var(--font-weight-regular);
  color: var(--color-text-primary);
}

.form-radio {
  margin-right: var(--spacing-xs);
}

.form-range {
  width: 100%;
  max-width: 300px;
}

.range-labels {
  display: flex;
  justify-content: space-between;
  width: 100%;
  max-width: 300px;
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.form-checkbox {
  margin-right: var(--spacing-s);
}

.form-actions {
  display: flex;
  gap: var(--spacing-m);
  padding-top: var(--spacing-m);
  border-top: 1px solid var(--color-border);
}

.save-button, .reset-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.save-button {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.reset-button {
  background-color: var(--color-surface);
  color: var(--color-text-primary);
}
</style>