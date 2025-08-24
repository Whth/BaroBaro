<template>
  <n-flex :size="24" vertical>
    <n-h1 v-text="$t('settings.title')"></n-h1>
    <n-card>
      <n-tabs v-model:value="activeTab" animated type="line">
        <n-tab-pane :tab="$t('settings.general')" name="general">
          <GeneralSettings/>
        </n-tab-pane>
        <n-tab-pane :tab="$t('settings.ui')" name="ui">
          <UIPreferences/>
        </n-tab-pane>
        <n-tab-pane :tab="$t('settings.versionInfo')" name="version">
          <VersionInfo/>
        </n-tab-pane>
      </n-tabs>
    </n-card>

    <n-float-button-group bottom="24" position="fixed" right="24">
      <n-float-button :title="$t('settings.save')" @click="apply_and_save_config">
        <n-icon>
          <SaveIcon/>
        </n-icon>
      </n-float-button>
      <n-float-button :title="$t('settings.reset')" @click="reset_config">
        <n-icon>
          <RefreshIcon/>
        </n-icon>
      </n-float-button>
    </n-float-button-group>
  </n-flex>


</template>

<script lang="ts" setup>
import { ref } from "vue";
import { Refresh as RefreshIcon, Save as SaveIcon } from "@vicons/ionicons5";
import GeneralSettings from "../../components/settings/GeneralSettings.vue";
import UIPreferences from "../../components/settings/UIPreferences.vue";
import VersionInfo from "../../components/settings/VersionInfo.vue";
import { config, reset_config } from "../../invokes.ts";
import { i18n } from "@/i18n.ts";
import { save_config } from "@/invokes.ts";
import { Language, languageToJSON, Theme } from "@/proto/config.ts";
import { currentTheme, theme_mapping } from "@/composables/useTheme.ts";

const activeTab = ref("general");

const apply_and_save_config = async () => {
	await save_config();
	i18n.global.locale.value = languageToJSON(
		config.value.uiConfig?.language ?? Language.en,
	);
	currentTheme.value =
		theme_mapping[config.value.uiConfig?.theme ?? Theme.Light];
};
</script>