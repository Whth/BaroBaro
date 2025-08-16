<template>
  <div class="profile-list">
    <div class="profile-list-header">
      <h2>Profiles</h2>
      <button class="add-profile-button" @click="createNewProfile">
        + New Profile
      </button>
    </div>
    <div class="profile-cards">
      <ProfileCard
        v-for="profile in profiles"
        :key="profile.id"
        :profile="profile"
        :is-active="profile.id === activeProfileId"
        @edit="editProfile"
        @delete="deleteProfile"
        @activate="activateProfile"
        @duplicate="duplicateProfile"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ProfileCard from './ProfileCard.vue'

interface Profile {
  id: string
  name: string
  gameId: string
  modCount: number
  createdAt: Date
}

const profiles = ref<Profile[]>([
  {
    id: '1',
    name: 'Survival Profile',
    gameId: 'minecraft',
    modCount: 12,
    createdAt: new Date('2025-01-15')
  },
  {
    id: '2',
    name: 'Creative Profile',
    gameId: 'minecraft',
    modCount: 8,
    createdAt: new Date('2025-02-20')
  },
  {
    id: '3',
    name: 'Adventure Profile',
    gameId: 'minecraft',
    modCount: 15,
    createdAt: new Date('2025-03-10')
  }
])

const activeProfileId = ref('1')

const createNewProfile = () => {
  console.log('Create new profile')
  // In a real app, this would emit an event to open the profile editor
}

const editProfile = (profileId: string) => {
  console.log('Edit profile:', profileId)
  // In a real app, this would emit an event to open the profile editor with the profile data
}

const deleteProfile = (profileId: string) => {
  const profile = profiles.value.find(p => p.id === profileId)
  if (profile) {
    if (confirm(`Are you sure you want to delete "${profile.name}"?`)) {
      profiles.value = profiles.value.filter(p => p.id !== profileId)
      console.log('Deleted profile:', profileId)
    }
  }
}

const activateProfile = (profileId: string) => {
  activeProfileId.value = profileId
  console.log('Activated profile:', profileId)
}

const duplicateProfile = (profileId: string) => {
  const profile = profiles.value.find(p => p.id === profileId)
  if (profile) {
    const newProfile = {
      ...profile,
      id: `${profile.id}-copy`,
      name: `${profile.name} (Copy)`,
      createdAt: new Date()
    }
    profiles.value.push(newProfile)
    console.log('Duplicated profile:', profileId)
  }
}

onMounted(() => {
  console.log('Profile list mounted')
})
</script>

<style scoped>
.profile-list {
  flex: 1;
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
  padding: var(--spacing-l);
}

.profile-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-l);
}

.profile-list-header h2 {
  margin: 0;
}

.add-profile-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.profile-cards {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}
</style>