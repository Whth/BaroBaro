<template>
  <div class="paths-settings">
    <h2>Paths Settings</h2>
    <div class="settings-form">
      <div class="form-group">
        <label for="game-path" class="form-label">Game Installation Path</label>
        <div class="path-input-group">
          <input
            id="game-path"
            v-model="settings.gamePath"
            type="text"
            class="form-input"
            placeholder="Select game installation path"
            readonly
          />
          <button class="browse-button" @click="browseGamePath">Browse</button>
        </div>
      </div>
      <div class="form-group">
        <label for="download-path" class="form-label">Mod Download Path</label>
        <div class="path-input-group">
          <input
            id="download-path"
            v-model="settings.downloadPath"
            type="text"
            class="form-input"
            placeholder="Select mod download path"
            readonly
          />
          <button class="browse-button" @click="browseDownloadPath">Browse</button>
        </div>
      </div>
      <div class="form-group">
        <label for="backup-path" class="form-label">Backup Path</label>
        <div class="path-input-group">
          <input
            id="backup-path"
            v-model="settings.backupPath"
            type="text"
            class="form-input"
            placeholder="Select backup path"
            readonly
          />
          <button class="browse-button" @click="browseBackupPath">Browse</button>
        </div>
      </div>
      <div class="form-actions">
        <button class="save-button" @click="saveSettings">Save Paths</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface PathsSettings {
  gamePath: string
  downloadPath: string
  backupPath: string
}

const settings = ref<PathsSettings>({
  gamePath: '',
  downloadPath: '',
  backupPath: ''
})

const browseGamePath = () => {
  console.log('Browsing for game path')
  // In a real app, this would open a file dialog using Tauri
  settings.value.gamePath = 'C:\\Program Files\\Game\\'
}

const browseDownloadPath = () => {
  console.log('Browsing for download path')
  // In a real app, this would open a file dialog using Tauri
  settings.value.downloadPath = 'C:\\Users\\User\\Downloads\\Mods\\'
}

const browseBackupPath = () => {
  console.log('Browsing for backup path')
  // In a real app, this would open a file dialog using Tauri
  settings.value.backupPath = 'C:\\Users\\User\\Documents\\ModBackups\\'
}

const saveSettings = () => {
  console.log('Saving paths settings:', settings.value)
  // In a real app, this would save to the backend or local storage
  alert('Paths settings saved successfully!')
}

onMounted(() => {
  console.log('Paths settings mounted')
})
</script>

<style scoped>
.paths-settings h2 {
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

.path-input-group {
  display: flex;
  gap: var(--spacing-s);
}

.form-input {
  flex: 1;
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.browse-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
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