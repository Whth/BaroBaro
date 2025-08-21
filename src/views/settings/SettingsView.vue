<template>
  <div class="settings-view">
    <h1>Settings</h1>
    <div class="settings-content">
      <div class="settings-sidebar">
        <ul class="settings-menu">
          <li
            v-for="section in settingsSections"
            :key="section.id"
            class="menu-item"
            :class="{ active: activeSection === section.id }"
            @click="setActiveSection(section.id)"
          >
            {{ section.name }}
          </li>
        </ul>
      </div>
      <div class="settings-main">
        <GeneralSettings v-if="activeSection === 'general'" />
        <PathsSettings v-else-if="activeSection === 'paths'" />
        <UIPreferences v-else-if="activeSection === 'ui'" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import GeneralSettings from "../../components/settings/GeneralSettings.vue";
import PathsSettings from "../../components/settings/PathsSettings.vue";
import UIPreferences from "../../components/settings/UIPreferences.vue";

interface SettingsSection {
	id: string;
	name: string;
}

const settingsSections: SettingsSection[] = [
	{ id: "general", name: "General" },
	{ id: "paths", name: "Paths" },
	{ id: "ui", name: "UI Preferences" },
];

const activeSection = ref("general");

const setActiveSection = (sectionId: string) => {
	activeSection.value = sectionId;
};
</script>

<style scoped>
.settings-view {
  padding: var(--spacing-l);
}

.settings-view h1 {
  margin: 0 0 var(--spacing-l) 0;
}

.settings-content {
  display: flex;
  gap: var(--spacing-l);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
  overflow: hidden;
  height: calc(100vh - 200px);
  min-height: 400px;
}

.settings-sidebar {
  width: 200px;
  border-right: 1px solid var(--color-border);
}

.settings-menu {
  list-style: none;
  padding: 0;
  margin: 0;
}

.menu-item {
  padding: var(--spacing-m);
  cursor: pointer;
  border-bottom: 1px solid var(--color-border);
  color: var(--color-text-primary);
}

.menu-item:hover {
  background-color: var(--color-background);
}

.menu-item.active {
  background-color: var(--color-primary);
  color: white;
}

.settings-main {
  flex: 1;
  padding: var(--spacing-l);
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--color-border) transparent;
}

.settings-main::-webkit-scrollbar {
  width: 8px;
}

.settings-main::-webkit-scrollbar-track {
  background: transparent;
}

.settings-main::-webkit-scrollbar-thumb {
  background-color: var(--color-border);
  border-radius: 4px;
}

.settings-main::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-text-secondary);
}

@media (max-width: 768px) {
  .settings-content {
    flex-direction: column;
  }

  .settings-sidebar {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--color-border);
  }
}
</style>
