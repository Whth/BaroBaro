<template>
  <n-config-provider :theme="currentTheme" autofocus>
    <n-message-provider>

      <div class="app-container">
        <Layout>
          <router-view/>
        </Layout>
      </div>
    </n-message-provider>
  </n-config-provider>
</template>

<script lang="ts" setup>
import { onBeforeMount } from "vue";
import Layout from "./components/core/Layout.vue";
import { Language, languageToJSON, Theme } from "./proto/config.ts";
import {
	config,
	list_enabled_mods,
	list_installed_mods,
	list_mod_lists,
	refresh_config,
	retrieve_mod_metadata,
} from "./invokes.ts";
import { currentTheme, theme_mapping } from "./composables/useTheme.ts";
import { i18n } from "./i18n.ts";

onBeforeMount(async () => {
	await refresh_config(),
		(currentTheme.value =
			theme_mapping[config.value.uiConfig?.theme ?? Theme.Light]);
	i18n.global.locale.value = languageToJSON(
		config.value.uiConfig?.language ?? Language.EN,
	);
	await Promise.all([
		list_installed_mods(),
		list_enabled_mods(),
		list_mod_lists(),
	]);
	await retrieve_mod_metadata();
});
</script>

