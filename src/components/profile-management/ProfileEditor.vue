<template>
  <div class="profile-editor">
    <h2>{{ isEditing ? "Edit Profile" : "Create New Profile" }}</h2>
    <form @submit.prevent="handleSaveProfile" class="profile-form">
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
        <label for="base-package" class="form-label">Base Package</label>
        <select
          id="base-package"
          v-model="profileData.basePackage"
          class="form-select"
          required
        >
          <option value="Vanilla">Vanilla</option>
          <option value="Experimental">Experimental</option>
        </select>
      </div>
      <div class="form-group">
        <label class="form-label">Mod Configuration</label>
        <div class="mod-config">
          <div
            v-for="mod in installed_mod"
            :key="mod.steamWorkshopId"
            class="mod-config-item"
          >
            <input
              :id="`mod-${mod.steamWorkshopId}`"
              type="checkbox"
              :value="mod.steamWorkshopId"
              v-model="profileData.enabledMods"
              class="mod-checkbox"
            />
            <label :for="`mod-${mod.steamWorkshopId}`" class="mod-label">
              <span class="mod-name">{{ mod.name }}</span>
              <span class="mod-version">{{ mod.modVersion }}</span>
            </label>
          </div>
        </div>
      </div>
      <div class="form-actions">
        <button type="button" class="cancel-button" @click="cancel">
          Cancel
        </button>
        <button type="submit" class="save-button">
          {{ isEditing ? "Update Profile" : "Create Profile" }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import type { Profile } from "../../types";
import { useModManager } from "../../composables/useModManager";

interface ProfileData {
  name: string;
  basePackage: string;
  enabledMods: string[];
}

const props = defineProps<{
  profile?: Profile;
}>();

const emit = defineEmits<{
  (e: "save", profile: Profile): void;
  (e: "cancel"): void;
}>();

const {
  installed_mod,
  refreshInstalledMods,
  saveProfile: saveProfileToBackend,
} = useModManager();

const isEditing = computed(() => !!props.profile);

const profileData = ref<ProfileData>({
  name: props.profile?.name || "",
  basePackage: props.profile?.basePackage || "Vanilla",
  enabledMods: props.profile?.enabledMods || [],
});

const handleSaveProfile = async () => {
  if (!profileData.value.name.trim()) return;

  const profile: Profile = {
    id: props.profile?.id || Date.now().toString(),
    name: profileData.value.name,
    basePackage: profileData.value.basePackage,
    enabledMods: profileData.value.enabledMods,
  };

  try {
    await saveProfileToBackend(profile);
    emit("save", profile);
  } catch (error) {
    console.error("Failed to save profile:", error);
    // TODO: Show error message to user
  }
};

const cancel = () => {
  emit("cancel");
};

// Load installed mods when component mounts
onMounted(() => {
  refreshInstalledMods();
});
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

.form-input,
.form-select {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.form-input:focus,
.form-select:focus {
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

.cancel-button,
.save-button {
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
