<template>
  <div class="profile-editor">
    <h2>{{ isEditing ? 'Edit Profile' : 'Create New Profile' }}</h2>
    <form @submit.prevent="saveProfile" class="profile-form">
      <div class="form-group">
        <label for="profile-name" class="form-label">Profile Name</label>
        <input
          id="profile-name"
          v-model="profileData.name"
          type="text"
          class="form-input"
          placeholder="Enter profile name"
          required
        />
      </div>
      <div class="form-group">
        <label for="game-id" class="form-label">Game</label>
        <select id="game-id" v-model="profileData.gameId" class="form-select" required>
          <option value="" disabled>Select a game</option>
          <option value="minecraft">Minecraft</option>
          <option value="terraria">Terraria</option>
          <option value="factorio">Factorio</option>
          <option value="stellaris">Stellaris</option>
        </select>
      </div>
      <div class="form-group">
        <label class="form-label">Mod Configuration</label>
        <div class="mod-config">
          <div v-for="mod in availableMods" :key="mod.id" class="mod-config-item">
            <input
              :id="`mod-${mod.id}`"
              type="checkbox"
              :value="mod.id"
              v-model="profileData.enabledMods"
              class="mod-checkbox"
            />
            <label :for="`mod-${mod.id}`" class="mod-label">
              <span class="mod-name">{{ mod.name }}</span>
              <span class="mod-version">{{ mod.version }}</span>
            </label>
          </div>
        </div>
      </div>
      <div class="form-actions">
        <button type="button" class="cancel-button" @click="cancel">
          Cancel
        </button>
        <button type="submit" class="save-button">
          {{ isEditing ? 'Update Profile' : 'Create Profile' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Mod {
  id: string
  name: string
  version: string
}

interface ProfileData {
  id?: string
  name: string
  gameId: string
  enabledMods: string[]
}

const isEditing = ref(false)

const profileData = ref<ProfileData>({
  name: '',
  gameId: '',
  enabledMods: []
})

const availableMods = ref<Mod[]>([
  { id: '1', name: 'Better Graphics', version: '1.3.0' },
  { id: '2', name: 'New Weapons Pack', version: '2.1.0' },
  { id: '3', name: 'Improved AI', version: '1.5.0' },
  { id: '4', name: 'Custom Maps', version: '1.0.2' },
  { id: '5', name: 'Enhanced Sound', version: '2.0.1' }
])

const saveProfile = () => {
  console.log('Saving profile:', profileData.value)
  // In a real app, this would call the backend to save the profile
  alert(isEditing.value ? 'Profile updated successfully!' : 'Profile created successfully!')
  resetForm()
}

const cancel = () => {
  resetForm()
  console.log('Cancelled profile editing')
}

const resetForm = () => {
  profileData.value = {
    name: '',
    gameId: '',
    enabledMods: []
  }
  isEditing.value = false
}

onMounted(() => {
  console.log('Profile editor mounted')
})
</script>

<style scoped>
.profile-editor {
  flex: 1;
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
  padding: var(--spacing-l);
  min-width: 300px;
}

.profile-editor h2 {
  margin: 0 0 var(--spacing-l) 0;
}

.profile-form {
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

.form-input, .form-select {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.form-input:focus, .form-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

.mod-config {
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  padding: var(--spacing-m);
  max-height: 300px;
  overflow-y: auto;
}

.mod-config-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-s) 0;
  border-bottom: 1px solid var(--color-border);
}

.mod-config-item:last-child {
  border-bottom: none;
}

.mod-checkbox {
  margin-right: var(--spacing-s);
}

.mod-label {
  display: flex;
  justify-content: space-between;
  flex: 1;
  cursor: pointer;
}

.mod-name {
  color: var(--color-text-primary);
}

.mod-version {
  color: var(--color-text-secondary);
  font-size: var(--font-size-body-small);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-m);
  padding-top: var(--spacing-m);
  border-top: 1px solid var(--color-border);
}

.cancel-button, .save-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.cancel-button {
  background-color: var(--color-surface);
  color: var(--color-text-primary);
}

.save-button {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}
</style>