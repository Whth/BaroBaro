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
import DownloadModTab from "./DownloadModTab.vue";
import UpdateModTab from "./UpdateModTab.vue";
import RemoveModTab from "./RemoveModTab.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const tabs = [
	{ id: "download", name: t("tabs.downloadMod"), component: DownloadModTab },
	{ id: "update", name: t("tabs.updateMod"), component: UpdateModTab },
	{ id: "remove", name: t("tabs.removeMod"), component: RemoveModTab },
];
const activeTab = ref("download");
</script>

