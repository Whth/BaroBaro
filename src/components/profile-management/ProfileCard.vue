<template>
  <div class="profile-card" :class="{ active: isActive }">
    <div class="profile-card-header">
      <h3 class="profile-name">{{ profile.profileName }}</h3>
      <div v-if="isActive" class="active-indicator">Active</div>
    </div>
    <div class="profile-card-content">
      <div class="profile-info">
        <div class="info-item">
          <span class="info-label">Base Package:</span>
          <span class="info-value">{{ profile.basePackage }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Mods:</span>
          <span class="info-value">{{ profile.mods.length }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">Created:</span>
          <span class="info-value">N/A</span>
        </div>
      </div>
    </div>
    <div class="profile-card-actions">
      <button class="action-button activate-button" @click="activateProfile">
        {{ isActive ? "Active" : "Activate" }}
      </button>
      <div class="more-actions">
        <button class="action-button edit-button" @click="editProfile">
          Edit
        </button>
        <button
          class="action-button duplicate-button"
          @click="duplicateProfile"
        >
          Duplicate
        </button>
        <button class="action-button delete-button" @click="deleteProfile">
          Delete
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import type { ModList } from "../../proto/mods";

const props = defineProps<{
	profile: ModList;
	isActive: boolean;
}>();

const emit = defineEmits<{
	(e: "edit", name: string): void;
	(e: "delete", name: string): void;
	(e: "activate", name: string): void;
	(e: "duplicate", name: string): void;
}>();

const _editProfile = () => {
	emit("edit", props.profile.profileName);
};

const _deleteProfile = () => {
	emit("delete", props.profile.profileName);
};

const _activateProfile = () => {
	emit("activate", props.profile.profileName);
};

const _duplicateProfile = () => {
	emit("duplicate", props.profile.profileName);
};
</script>

<style scoped>
.profile-card {
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  padding: var(--spacing-m);
  background-color: var(--color-surface);
  transition: all 0.2s ease;
}

.profile-card:hover {
  box-shadow: var(--shadow-level-2);
  border-color: var(--color-primary-light);
}

.profile-card.active {
  border-color: var(--color-primary);
  background-color: rgba(59, 130, 246, 0.05);
}

.profile-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-s);
}

.profile-name {
  margin: 0;
  font-size: var(--font-size-heading-3);
  color: var(--color-text-primary);
}

.active-indicator {
  padding: var(--spacing-xs) var(--spacing-s);
  background-color: var(--color-success);
  color: white;
  border-radius: var(--border-radius-soft);
  font-size: var(--font-size-caption);
  font-weight: var(--font-weight-medium);
}

.profile-card-content {
  margin-bottom: var(--spacing-m);
}

.profile-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.info-item {
  display: flex;
  justify-content: space-between;
}

.info-label {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.info-value {
  font-size: var(--font-size-body-small);
  color: var(--color-text-primary);
  font-weight: var(--font-weight-medium);
}

.profile-card-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.activate-button {
  padding: var(--spacing-xs) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
  font-size: var(--font-size-body-small);
}

.activate-button:disabled {
  background-color: var(--color-text-secondary);
  cursor: not-allowed;
}

.more-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.action-button {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-size: var(--font-size-body-small);
  font-weight: var(--font-weight-medium);
}

.edit-button:hover {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.duplicate-button:hover {
  background-color: var(--color-info);
  color: white;
  border-color: var(--color-info);
}

.delete-button:hover {
  background-color: var(--color-error);
  color: white;
  border-color: var(--color-error);
}
</style>
