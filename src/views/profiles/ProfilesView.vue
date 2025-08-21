<template>
  <div class="profiles-view">
    <h1>Profile Management</h1>
    <div class="profiles-content">
      <ProfileList @create="handleCreateProfile" @edit="handleEditProfile" />
      <ProfileEditor
        v-if="showEditor"
        :profile="editingProfile"
        @save="handleSaveProfile"
        @cancel="handleCancelEdit"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import type { Profile } from "../../types";
import { useModManager } from "../../composables/useModManager";

const { getModListByName } = useModManager();

const showEditor = ref(false);
const editingProfile = ref<Profile | null>(null);

const _handleCreateProfile = () => {
	editingProfile.value = null;
	showEditor.value = true;
};

const _handleEditProfile = (profileName: string) => {
	const modList = getModListByName(profileName);
	if (modList) {
		editingProfile.value = {
			id: profileName,
			name: modList.profileName,
			basePackage: modList.basePackage,
			enabledMods: modList.mods,
		};
		showEditor.value = true;
	}
};

const _handleSaveProfile = (_profile: Profile) => {
	// Profile is saved through the composable, so we just need to hide the editor
	showEditor.value = false;
	editingProfile.value = null;
};

const _handleCancelEdit = () => {
	showEditor.value = false;
	editingProfile.value = null;
};
</script>

<style scoped>
.profiles-view {
  padding: var(--spacing-l);
}

.profiles-view h1 {
  margin: 0 0 var(--spacing-l) 0;
}

.profiles-content {
  display: flex;
  gap: var(--spacing-l);
}

@media (max-width: 768px) {
  .profiles-content {
    flex-direction: column;
  }
}
</style>
