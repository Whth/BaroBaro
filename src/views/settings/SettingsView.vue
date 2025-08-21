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
        <VersionInfo v-else-if="activeSection === 'version'" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import GeneralSettings from "../../components/settings/GeneralSettings.vue";
import PathsSettings from "../../components/settings/PathsSettings.vue";
import UIPreferences from "../../components/settings/UIPreferences.vue";
import VersionInfo from "../../components/settings/VersionInfo.vue";

interface SettingsSection {
	id: string;
	name: string;
}

const settingsSections: SettingsSection[] = [
	{ id: "general", name: "General" },
	{ id: "paths", name: "Paths" },
	{ id: "ui", name: "UI Preferences" },
	{ id: "version", name: "Version Info" },
];

// Note: These will be internationalized later when we add i18n to this component

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
	transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	position: relative;
	opacity: 0;
	transform: translateY(10px);
	animation: fadeInUp 0.6s ease-out forwards;
}

.menu-item:nth-child(1) { animation-delay: 0.1s; }
.menu-item:nth-child(2) { animation-delay: 0.2s; }
.menu-item:nth-child(3) { animation-delay: 0.3s; }

.menu-item:hover {
	background-color: var(--color-background);
	transform: translateX(8px);
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.menu-item.active {
	background: linear-gradient(135deg, var(--color-primary), var(--color-primary-dark));
	color: white;
	box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.menu-item.active::after {
	content: '';
	position: absolute;
	right: 0;
	top: 50%;
	transform: translateY(-50%);
	width: 4px;
	height: 60%;
	background: white;
	border-radius: 2px 0 0 2px;
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
