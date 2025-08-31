<template>
  <TitledPage>
    <template #title>
      <n-h1 v-text="$t('settings.title')"></n-h1>
    </template>
    <n-card>
      <n-tabs v-model:value="activeTab" animated type="line">
        <n-tab-pane :tab="$t('settings.general')" name="general">
          <n-scrollbar style="height: 72vh">
            <GeneralSettings/>
          </n-scrollbar>
        </n-tab-pane>
        <n-tab-pane :tab="$t('settings.ui')" name="ui">
          <n-scrollbar style="height: 72vh">
            <UIPreferences/>
          </n-scrollbar>
        </n-tab-pane>
        <n-tab-pane :tab="$t('settings.versionInfo')" name="version">
          <n-scrollbar style="height: 72vh">
            <VersionInfo/>
          </n-scrollbar>
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
  </TitledPage>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { Refresh as RefreshIcon, Save as SaveIcon } from "@vicons/ionicons5";
import { config, reset_config, save_config } from "../../invokes.ts";
import { i18n } from "../../i18n.ts";
import { Language, languageToJSON } from "../../proto/config.ts";
import {
	setBackgroundImage,
	setTheme,
	setTransparent,
} from "../../composables/useTheme.ts";
import TitledPage from "../../components/core/TitledPage.vue";
import VersionInfo from "../../components/settings/VersionInfo.vue";
import UIPreferences from "../../components/settings/UIPreferences.vue";
import GeneralSettings from "../../components/settings/GeneralSettings.vue";
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";

const activeTab = ref("general");
const { t } = useI18n();

const message = useMessage();

const apply_and_save_config = async () => {
	await save_config();
	message.success(t("settings.saved"));
	i18n.global.locale.value = languageToJSON(
		config.value.uiConfig?.language ?? Language.EN,
	);
	setTransparent();
	setTheme();
	await setBackgroundImage();
};
</script>