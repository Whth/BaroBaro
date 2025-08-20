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
import { useModManager } from '../../composables/useModManager'
import { Config } from '../../proto/config'
import { open } from '@tauri-apps/plugin-dialog'

const { config, updateGameHome, updateSteamCmdHome } = useModManager()

const settings = ref({
  gamePath: '',
  downloadPath: '',
  backupPath: ''
})
const browseGamePath = async () => {
  console.log('Browsing for game path')
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Barotrauma Game Installation Path'
    })
    
    if (selected) {
      settings.value.gamePath = selected as string
      
      // Update the game home in the config
      await updateGameHome(selected as string)
      console.log('Game path updated successfully!')
    }
  } catch (error) {
    console.error('Failed to select game path:', error)
    // TODO: Show error message to user
  }
}

const browseDownloadPath = async () => {
  console.log('Browsing for download path')
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Mod Download Path'
    })
    
    if (selected) {
      settings.value.downloadPath = selected as string
      
      // Update the SteamCMD home in the config
      await updateSteamCmdHome(selected as string)
      console.log('Download path updated successfully!')
    }
  } catch (error) {
    console.error('Failed to select download path:', error)
    // TODO: Show error message to user
  }
}

const browseBackupPath = async () => {
  console.log('Browsing for backup path')
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Backup Path'
    })
    
    if (selected) {
      settings.value.backupPath = selected as string
      console.log('Backup path selected successfully!')
    }
  } catch (error) {
    console.error('Failed to select backup path:', error)
    // TODO: Show error message to user
  }
}

const saveSettings = async () => {
  try {
    console.log('Saving paths settings:', settings.value)
    
    // Update game home
    if (settings.value.gamePath) {
      await updateGameHome(settings.value.gamePath)
    }
    
    // Update SteamCMD home (download path)
    if (settings.value.downloadPath) {
      await updateSteamCmdHome(settings.value.downloadPath)
    }
    
    console.log('Paths settings saved successfully!')
  } catch (error) {
    console.error('Failed to save paths settings:', error)
    // TODO: Show error message to user
  }
}

onMounted(() => {
  // Initialize settings with values from config
  if (config.value) {
    settings.value.gamePath = config.value.gameHome
    settings.value.downloadPath = config.value.steamcmdHome
    // Backup path is not in the config, so we'll leave it as empty for now
  }
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