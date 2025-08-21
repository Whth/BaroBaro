<template>
  <div
    class="mod-card"
    :class="{
      'mod-card-disabled': !isEnabled,
      'drag-over': isDragOver,
    }"
    :draggable="draggable"
    @dragstart="$emit('dragstart', $event)"
    @dragover="$emit('dragover', $event)"
    @dragenter="$emit('dragenter')"
    @dragleave="$emit('dragleave')"
    @drop="$emit('drop', $event)"
    @dragend="$emit('dragend')"
  >
    <div class="mod-card-header">
      <div class="mod-header-content">
        <div class="mod-priority-indicator">
          <span class="priority-number">{{ index + 1 }}</span>
        </div>
        <h3 class="mod-name">{{ mod.name }}</h3>
      </div>
      <div class="mod-actions">
        <button
          class="toggle-button"
          :class="{ 'toggle-enabled': isEnabled }"
          @click="toggleMod"
        >
          {{ isEnabled ? "Enabled" : "Disabled" }}
        </button>
      </div>
    </div>
    <div class="mod-card-content">
      <p class="mod-version">Version: {{ mod.modVersion }}</p>
      <p class="mod-author">Steam ID: {{ mod.steamWorkshopId }}</p>
      <p class="mod-description">Game Version: {{ mod.gameVersion }}</p>
    </div>
    <div class="mod-card-footer">
      <span
        class="mod-source"
        :class="`source-${mod.corePackage ? 'core' : 'mod'}`"
      >
        {{ mod.corePackage ? "Core Package" : "Mod" }}
      </span>
      <button class="details-button" @click="selectMod">View Details</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from "vue";
import { BarotraumaMod } from "../../proto/mods";

const props = defineProps<{
  mod: BarotraumaMod;
  index: number;
  draggable?: boolean;
  isDragOver?: boolean;
  isEnabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "toggleMod", id: string): void;
  (e: "selectMod", id: string): void;
  (e: "dragstart", event: DragEvent): void;
  (e: "dragover", event: DragEvent): void;
  (e: "dragenter"): void;
  (e: "dragleave"): void;
  (e: "drop", event: DragEvent): void;
  (e: "dragend"): void;
}>();

const toggleMod = () => {
  emit("toggleMod", props.mod.steamWorkshopId);
};

const selectMod = () => {
  emit("selectMod", props.mod.steamWorkshopId);
};
</script>

<style scoped>
.mod-card {
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  padding: var(--spacing-m);
  background-color: var(--color-surface);
  transition: all 0.2s ease;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.mod-card:hover {
  box-shadow: var(--shadow-level-2);
  border-color: var(--color-primary-light);
}

.mod-card-disabled {
  opacity: 0.7;
}

.drag-over {
  border-color: var(--color-primary);
  background-color: var(--color-primary-light);
}

.mod-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-s);
}

.mod-header-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
}

.mod-priority-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background-color: var(--color-primary);
  color: white;
  font-size: var(--font-size-body-small);
  font-weight: var(--font-weight-bold);
}

.mod-name {
  margin: 0;
  font-size: var(--font-size-heading-3);
  color: var(--color-text-primary);
}

.mod-actions {
  display: flex;
  gap: var(--spacing-s);
}

.toggle-button {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-size: var(--font-size-body-small);
  font-weight: var(--font-weight-medium);
}

.toggle-enabled {
  background-color: var(--color-success);
  color: white;
  border-color: var(--color-success);
}

.mod-card-content {
  margin-bottom: var(--spacing-m);
}

.mod-version {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
  margin: 0 0 var(--spacing-xs) 0;
}

.mod-author {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
  margin: 0 0 var(--spacing-m) 0;
  font-style: italic;
}

.mod-description {
  margin: 0;
  color: var(--color-text-primary);
  line-height: 1.4;
}

.mod-card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.mod-source {
  padding: var(--spacing-xs) var(--spacing-s);
  border-radius: var(--border-radius-soft);
  font-size: var(--font-size-caption);
  font-weight: var(--font-weight-medium);
}

.source-local {
  background-color: var(--color-primary-light);
  color: var(--color-primary-dark);
}

.source-remote {
  background-color: var(--color-info);
  color: white;
}

.details-button {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-size: var(--font-size-body-small);
}
</style>
