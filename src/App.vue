<template>

  <n-config-provider
      :style="{ backgroundImage: `url(${currentBackgroundImage})`, backgroundSize: 'cover'}"
      :theme="currentTheme"
      :theme-overrides="currentThemeOverrides"
      autofocus class="glass-card">
    <n-message-provider>

      <Layout>
        <router-view/>


      </Layout>

    </n-message-provider>
  </n-config-provider>

</template>

<script lang="ts" setup>
import { onBeforeMount, ref, Ref } from "vue";
import Layout from "./components/core/Layout.vue";
import { Language, languageToJSON } from "./proto/config.ts";
import {
	config,
	get_background_image,
	list_enabled_mods,
	list_installed_mods,
	list_mod_lists,
	refresh_config,
	retrieve_mod_metadata,
} from "./invokes.ts";
import {
	currentBackgroundImage,
	currentTheme,
	currentThemeOverrides,
	setBackgroundImage,
	setTheme,
	setTransparent,
} from "./composables/useTheme.ts";
import { i18n } from "./i18n.ts";

const image: Ref<string | null> = ref("");

onBeforeMount(async () => {
	try {
		const bgImage = await get_background_image();
		if (bgImage) {
			image.value = bgImage;
		}
	} catch (err) {
		console.error("Failed to load background image:", err);
	}
});

onBeforeMount(async () => {
	await refresh_config();
	setTransparent();
	setTheme();
	await setBackgroundImage();
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

