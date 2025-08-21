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
import type { BarotraumaMod } from "../../proto/mods";

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
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  animation: fadeInUp 0.6s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  opacity: 0;
}

.mod-card:nth-child(1) {
  animation-delay: 0.1s;
}

.mod-card:nth-child(2) {
  animation-delay: 0.2s;
}

.mod-card:nth-child(3) {
  animation-delay: 0.3s;
}

.mod-card:nth-child(4) {
  animation-delay: 0.4s;
}

.mod-card:nth-child(5) {
  animation-delay: 0.5s;
}

.mod-card:hover {
	transform: translateY(-10px) scale(1.03);
	box-shadow: 0 25px 50px rgba(0, 0, 0, 0.2);
	border-color: var(--color-primary-light);
}

.mod-card:active {
	transform: translateY(-5px) scale(1.01);
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
	transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
	position: relative;
	overflow: hidden;
}

.toggle-button::before {
	content: '';
	position: absolute;
	top: 0;
	left: -100%;
	width: 100%;
	height: 100%;
	background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
	transition: left 0.5s;
}

.toggle-button:hover::before {
	left: 100%;
}

.toggle-button:hover {
	transform: translateY(-2px);
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
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
