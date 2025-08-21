<template>
  <div class="mod-management-tabs">
    <div class="tabs-header">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="tab-button"
        :class="{ active: activeTab === tab.id }"
        @click="setActiveTab(tab.id)"
      >
        {{ tab.name }}
      </button>
    </div>
    <div class="tabs-content">
      <InstallModTab v-if="activeTab === 'install'" />
      <DownloadModTab v-else-if="activeTab === 'download'" />
      <UpdateModTab v-else-if="activeTab === 'update'" />
      <RemoveModTab v-else-if="activeTab === 'remove'" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import InstallModTab from "./InstallModTab.vue";
import DownloadModTab from "./DownloadModTab.vue";
import UpdateModTab from "./UpdateModTab.vue";
import RemoveModTab from "./RemoveModTab.vue";

interface Tab {
  id: string;
  name: string;
}

const tabs: Tab[] = [
  { id: "install", name: "Install Mod" },
  { id: "download", name: "Download Mod" },
  { id: "update", name: "Update Mod" },
  { id: "remove", name: "Remove Mod" },
];

const activeTab = ref("install");

const setActiveTab = (tabId: string) => {
  activeTab.value = tabId;
};
</script>

<style scoped>
.mod-management-tabs {
  background-color: var(--color-surface);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-1);
  overflow: hidden;
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid var(--color-border);
  background-color: var(--color-background);
}

.tab-button {
  flex: 1;
  padding: var(--spacing-m) var(--spacing-l);
  border: none;
  background: none;
  cursor: pointer;
  font-size: var(--font-size-body-regular);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-secondary);
  transition: all 0.2s ease;
}

.tab-button:hover {
  background-color: var(--color-surface);
}

.tab-button.active {
  color: var(--color-primary);
  border-bottom: 2px solid var(--color-primary);
  background-color: var(--color-surface);
}

.tabs-content {
  padding: var(--spacing-l);
}
</style>
