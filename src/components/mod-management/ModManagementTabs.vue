<template>
  <n-tabs
      v-model:value="activeTab"
      animated
      justify-content="space-evenly"
      style="height: 100%"
      type="line"
  >

    <n-tab-pane
        v-for="tab in tabs"
        :key="tab.id"
        :name="tab.id"
        :tab="tab.name"
        style="height: 100%">

      <component :is="tab.component"/>
    </n-tab-pane>
  </n-tabs>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import DownloadModTab from "./DownloadModTab.vue";
import ProfilesTab from "./ProfilesTab.vue";
import RemoveModTab from "./RemoveModTab.vue";
import UpdateModTab from "./UpdateModTab.vue";
import WorkshopBrowse from "./WorkshopBrowse.vue";

const { t } = useI18n();
const tabs = computed(() => [
	{ id: "browse", name: t("tabs.browse"), component: WorkshopBrowse },
	{ id: "download", name: t("tabs.downloadMod"), component: DownloadModTab },
	{ id: "update", name: t("tabs.updateMod"), component: UpdateModTab },
	{ id: "remove", name: t("tabs.removeMod"), component: RemoveModTab },
	{ id: "profiles", name: t("tabs.profiles"), component: ProfilesTab },
]);
const activeTab = ref("browse");
</script>

