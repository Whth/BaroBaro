<template>
  <div class="general-settings">
    <h2>General Settings</h2>
    <div class="settings-form">
      <div class="form-group">
        <label for="language" class="form-label">Language</label>
        <select id="language" v-model="settings.language" class="form-select">
          <option value="en">English</option>
          <option value="es">Spanish</option>
          <option value="fr">French</option>
          <option value="de">German</option>
          <option value="zh">Chinese</option>
        </select>
      </div>
      <div class="form-group">
        <label for="theme" class="form-label">Theme</label>
        <select id="theme" v-model="settings.theme" class="form-select">
          <option value="light">Light</option>
          <option value="dark">Dark</option>
          <option value="system">System</option>
        </select>
      </div>
      <div class="form-group">
        <label for="auto-check-updates" class="form-label">
          <input
            id="auto-check-updates"
            v-model="settings.autoCheckUpdates"
            type="checkbox"
            class="form-checkbox"
          />
          Automatically check for updates
        </label>
      </div>
      <div class="form-group">
        <label for="enable-notifications" class="form-label">
          <input
            id="enable-notifications"
            v-model="settings.enableNotifications"
            type="checkbox"
            class="form-checkbox"
          />
          Enable notifications
        </label>
      </div>
      <div class="form-actions">
        <button class="save-button" @click="saveSettings">Save Settings</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface GeneralSettings {
  language: string
  theme: string
  autoCheckUpdates: boolean
  enableNotifications: boolean
}

const settings = ref<GeneralSettings>({
  language: 'en',
  theme: 'system',
  autoCheckUpdates: true,
  enableNotifications: true
})

const saveSettings = () => {
  console.log('Saving general settings:', settings.value)
  // In a real app, this would save to the backend or local storage
  alert('General settings saved successfully!')
}

onMounted(() => {
  console.log('General settings mounted')
})
</script>

<style scoped>
.general-settings h2 {
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

.form-checkbox {
  margin-right: var(--spacing-s);
}

.form-actions {
  padding-top: var(--spacing-m);
  border-top: 1px solid var(--color-border);
}

.save-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
  width: fit-content;
}
</style>