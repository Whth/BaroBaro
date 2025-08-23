<template>
  <div class="s">
    <n-space :size="24" vertical>
      <n-h1 v-text="$t('settings.title')"></n-h1>
      <n-card>
        <n-tabs v-model:value="activeTab" animated type="line">
          <n-tab-pane :tab="$t('settings.general')" name="general">
            <GeneralSettings/>
          </n-tab-pane>
          <n-tab-pane :tab="$t('settings.paths')" name="paths">
            <PathsSettings/>
          </n-tab-pane>
          <n-tab-pane :tab="$t('settings.ui')" name="ui">
            <UIPreferences/>
          </n-tab-pane>
          <n-tab-pane :tab="$t('settings.versionInfo')" name="version">
            <VersionInfo/>
          </n-tab-pane>
        </n-tabs>
      </n-card>
    </n-space>
  </div>
</template>

<script lang="ts" setup>
import {ref} from "vue";
import GeneralSettings from "../../components/settings/GeneralSettings.vue";
import PathsSettings from "../../components/settings/PathsSettings.vue";
import UIPreferences from "../../components/settings/UIPreferences.vue";
import VersionInfo from "../../components/settings/VersionInfo.vue";

const activeTab = ref("general");
</script>

