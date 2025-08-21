<template>
  <div class="settings-view">
    <n-space vertical :size="24">
      <div class="settings-header">
        <h1 class="page-title">Settings</h1>
      </div>

      <n-card>
        <n-tabs v-model:value="activeTab" type="line" animated>
          <n-tab-pane name="general" tab="General">
            <GeneralSettings />
          </n-tab-pane>
          <n-tab-pane name="paths" tab="Paths">
            <PathsSettings />
          </n-tab-pane>
          <n-tab-pane name="ui" tab="UI Preferences">
            <UIPreferences />
          </n-tab-pane>
          <n-tab-pane name="version" tab="Version Info">
            <VersionInfo />
          </n-tab-pane>
        </n-tabs>
      </n-card>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NTabs, NTabPane, NSpace, NCard } from "naive-ui";
import GeneralSettings from "../../components/settings/GeneralSettings.vue";
import PathsSettings from "../../components/settings/PathsSettings.vue";
import UIPreferences from "../../components/settings/UIPreferences.vue";
import VersionInfo from "../../components/settings/VersionInfo.vue";

const activeTab = ref("general");
</script>

<style scoped>
.settings-view {
  flex: 1;
  overflow-y: auto;
}

.settings-header {
  margin-bottom: 24px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  color: var(--n-color-text-primary);
  margin: 0;
}

:deep(.n-card) {
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

:deep(.n-tabs) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.n-tabs-nav) {
  margin: 0;
  padding: 0 24px;
}

:deep(.n-tabs-content) {
  overflow-y: auto;
  padding: 24px;
}

:deep(.n-tabs-content::-webkit-scrollbar) {
  width: 8px;
}

:deep(.n-tabs-content::-webkit-scrollbar-track) {
  background: transparent;
}

:deep(.n-tabs-content::-webkit-scrollbar-thumb) {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

:deep(.n-tabs-content::-webkit-scrollbar-thumb:hover) {
  background-color: rgba(0, 0, 0, 0.3);
}
</style>
