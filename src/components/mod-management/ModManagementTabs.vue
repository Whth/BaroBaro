<template>
  <n-card :bordered="false" class="mod-management-card">
    <n-tabs
        v-model:value="activeTab"
        animated
        justify-content="space-evenly"
        type="line"
    >
      <n-tab-pane
          v-for="tab in tabs"
          :key="tab.id"
          :name="tab.id"
          :tab="tab.name"
      >
        <component :is="tab.component"/>
      </n-tab-pane>
    </n-tabs>
  </n-card>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import InstallModTab from "./InstallModTab.vue";
import DownloadModTab from "./DownloadModTab.vue";
import UpdateModTab from "./UpdateModTab.vue";
import RemoveModTab from "./RemoveModTab.vue";

// 定义标签页配置
const tabs = [
	{ id: "install", name: "Install Mod", component: InstallModTab },
	{ id: "download", name: "Download Mod", component: DownloadModTab },
	{ id: "update", name: "Update Mod", component: UpdateModTab },
	{ id: "remove", name: "Remove Mod", component: RemoveModTab },
];

// 当前激活的标签页
const activeTab = ref("install");
</script>

<style scoped>
.mod-management-card {
  height: 100%;
}

:deep(.n-tabs-pane-wrapper) {
  margin-top: 20px;
}
</style>