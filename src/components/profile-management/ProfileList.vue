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
        v-for="profile in mod_lists"
        :key="profile.profileName"
        :profile="profile"
        :is-active="profile.profileName === activeProfileName"
        @edit="editProfile"
        @delete="deleteProfile"
        @activate="activateProfile"
        @duplicate="duplicateProfile"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import ProfileCard from "./ProfileCard.vue";
import { useModManager } from "../../composables/useModManager";

const { mod_lists, refreshModLists, deleteModList } = useModManager();

const activeProfileName = ref("");

const emit = defineEmits<{
  (e: "create"): void;
  (e: "edit", profileName: string): void;
}>();

const createNewProfile = () => {
  emit("create");
};

const editProfile = (profileName: string) => {
  emit("edit", profileName);
};

const deleteProfile = async (profileName: string) => {
  const profile = mod_lists.value.find((p) => p.profileName === profileName);
  if (profile) {
    if (confirm(`Are you sure you want to delete "${profile.profileName}"?`)) {
      try {
        await deleteModList(profileName);
        console.log("Deleted profile:", profileName);
      } catch (error) {
        console.error("Failed to delete profile:", error);
        // TODO: Show error message to user
      }
    }
  }
};

const activateProfile = (profileName: string) => {
  activeProfileName.value = profileName;
  console.log("Activated profile:", profileName);
};

const duplicateProfile = (profileName: string) => {
  const profile = mod_lists.value.find((p) => p.profileName === profileName);
  if (profile) {
    // In a real implementation, this would call a Tauri command to duplicate the profile
    console.log("Duplicated profile:", profileName);
  }
};

onMounted(async () => {
  // Refresh mod lists when component mounts
  await refreshModLists();

  // Set the first mod list as the default active profile
  if (mod_lists.value.length > 0) {
    activeProfileName.value = mod_lists.value[0].profileName;
  }
  console.log("Profile list mounted with", mod_lists.value.length, "profiles");
});
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
