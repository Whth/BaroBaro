<template>
  <div class="paths-settings">
    <h2>Paths Settings</h2>
    <div class="settings-form">
      <div class="form-group">
        <label class="form-label" for="game-path">Game Installation Path</label>
        <div class="path-input-group">
          <input
              id="game-path"
              v-model="settings.gamePath"
              class="form-input"
              placeholder="Select game installation path"
              readonly
              type="text"
          />
          <button class="browse-button" @click="browseGamePath">Browse</button>
        </div>
      </div>
      <div class="form-group">
        <label class="form-label" for="steamcmd-path">SteamCMD Path</label>
        <div class="path-input-group">
          <input
              id="steamcmd-path"
              v-model="settings.steamcmdPath"
              class="form-input"
              placeholder="Select SteamCMD path"
              readonly
              type="text"
          />
          <button class="browse-button" @click="browseSteamCmdPath">Browse</button>
        </div>
      </div>
      <div class="form-actions">
        <button class="save-button" @click="saveSettings">Save Paths</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, ref} from 'vue'
import {useModManager} from '../../composables/useModManager'
import {message, open} from '@tauri-apps/plugin-dialog'

const {config, updateGameHome, updateSteamCmdHome} = useModManager()

const settings = ref({
  gamePath: '',
  steamcmdPath: ''
})

const showError = async (title: string, error: any) => {
  console.error(title, error)
  await message(`Error: ${error.message || error}`, {title, kind: 'error'})
}

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
    await showError('Failed to select game path', error)
  }
}

const browseSteamCmdPath = async () => {
  console.log('Browsing for SteamCMD path')
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select SteamCMD Path'
    })

    if (selected) {
      settings.value.steamcmdPath = selected as string

      // Update the SteamCMD home in the config
      await updateSteamCmdHome(selected as string)
      console.log('SteamCMD path updated successfully!')
    }
  } catch (error) {
    await showError('Failed to select SteamCMD path', error)
  }
}

const saveSettings = async () => {
  try {
    console.log('Saving paths settings:', settings.value)

    // Update game home
    if (settings.value.gamePath) {
      await updateGameHome(settings.value.gamePath)
    }

    // Update SteamCMD home
    if (settings.value.steamcmdPath) {
      await updateSteamCmdHome(settings.value.steamcmdPath)
    }

    console.log('Paths settings saved successfully!')
    await message('Paths settings saved successfully!', {title: 'Success', kind: 'info'})
  } catch (error) {
    await showError('Failed to save paths settings', error)
  }
}

onMounted(() => {
  // Initialize settings with values from config
  if (config.value) {
    settings.value.gamePath = config.value.gameHome
    settings.value.steamcmdPath = config.value.steamcmdHome
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