<template>
  <n-grid :cols="1" :x-gap="16" :y-gap="24">
    <n-gi>
      <!-- Language and Theme Settings -->
      <n-card class="form-section animate-fade-in">
        <template #header>
          <n-flex align="center" :size="12">
            <n-icon :size="20">
              <settings-outline />
            </n-icon>
            <n-text strong style="font-size: var(--text-lg);">
              {{ $t('settings.appearanceLanguage') }}
            </n-text>
          </n-flex>
        </template>
        
        <n-flex vertical :size="16">
          <n-form-item :label="$t('settings.languageLabel')" label-placement="left">
            <n-select
                v-model:value="languageModel"
                :options="[
                  { label: lang_show_mapping[Language.EN], value: Language.EN },
                  { label: lang_show_mapping[Language.ZH], value: Language.ZH }
                ]"
            />
          </n-form-item>

          <n-form-item :label="$t('settings.themeLabel')" label-placement="left">
            <n-select
                v-model:value="themeModel"
                :options="
                [
                  { label: $t('settings.light'), value: Theme.Light },
                  { label: $t('settings.dark'), value: Theme.Dark }
                ]"
            />
          </n-form-item>
        </n-flex>
      </n-card>
    </n-gi>

    <n-gi>
      <!-- Background Customization -->
      <n-card class="form-section animate-fade-in">
        <template #header>
          <n-flex align="center" :size="12">
            <n-icon :size="20">
              <image-outline />
            </n-icon>
            <n-text strong style="font-size: var(--text-lg);">
              {{ $t('settings.backgroundCustomization') }}
            </n-text>
          </n-flex>
        </template>

        <n-flex vertical :size="16">
          <n-form-item :label="$t('settings.backgroundImage')" label-placement="left">
            <n-input
                v-model:value="backgroundImageModel"
                :placeholder="$t('settings.noImageSelected')"
                readonly
                style="margin-right: 10px;"
            />
            <n-button @click="selectBackgroundImage">
              {{ $t('settings.browseForImage') }}
            </n-button>
            <n-button
                v-if="backgroundImageModel"
                class="clear-button"
                style="margin-left: 10px;"
                type="error"
                @click="clearBackgroundImage"
            >
              {{ $t('settings.clear') }}
            </n-button>
          </n-form-item>

          <n-form-item :label="$t('settings.backgroundOpacity')" label-placement="left">
            <n-slider
                v-model:value="backgroundOpacityModel"
                :max="1"
                :min="0"
                :step="0.01"
            />
          </n-form-item>

          <n-form-item :label="$t('settings.backgroundBlur')" label-placement="left">
            <n-slider
                v-model:value="backgroundBlurModel"
                :max="100"
                :min="0"
                :step="1"
            />
          </n-form-item>

          <n-form-item :label="$t('settings.foregroundOpacity')" label-placement="left">
            <n-slider
                v-model:value="foregroundOpacityModel"
                :max="1"
                :min="0"
                :step="0.01"
            />
          </n-form-item>
        </n-flex>
      </n-card>
    </n-gi>
  </n-grid>
</template>

<script lang="ts" setup>
import { ImageOutline, SettingsOutline } from "@vicons/ionicons5";
import { open } from "@tauri-apps/plugin-dialog";
import { computed, onMounted } from "vue";
import {
	setTheme,
	setTransparent,
} from "../../composables/useTheme.ts";
import { config, refresh_config } from "../../invokes.ts";
import {
	Language,
	Theme,
	UIConfig,
} from "../../proto/config.ts";


// Language mappings
const lang_show_mapping: Record<Language, string> = {
	[Language.EN]: "English",
	[Language.ZH]: "中文",
	[Language.UNRECOGNIZED]: "Unknown",
};

// Language model
const languageModel = computed({
	get: () => config.value.uiConfig?.language ?? Language.EN,
	set: (value) => {
		if (!config.value.uiConfig) {
			config.value.uiConfig = UIConfig.fromPartial({ language: value });
		} else {
			config.value.uiConfig.language = value;
		}
		setTheme();
	},
});

// Theme model
const themeModel = computed({
	get: () => config.value.uiConfig?.theme ?? Theme.Light,
	set: (value) => {
		if (!config.value.uiConfig) {
			config.value.uiConfig = UIConfig.fromPartial({ theme: value });
		} else {
			config.value.uiConfig.theme = value;
		}
		setTheme();
	},
});

// Background image model
const backgroundImageModel = computed({
	get: () => config.value.uiConfig?.backgroundImage ?? "",
	set: (value) => {
		if (!config.value.uiConfig) {
			config.value.uiConfig = UIConfig.fromPartial({ backgroundImage: value });
		} else {
			config.value.uiConfig.backgroundImage = value;
		}
	},
});

// Background opacity model
const backgroundOpacityModel = computed({
	get: () => config.value.uiConfig?.backgroundOpacity ?? 0.5,
	set: (value) => {
		if (!config.value.uiConfig) {
			config.value.uiConfig = UIConfig.fromPartial({ backgroundOpacity: value });
		} else {
			config.value.uiConfig.backgroundOpacity = value;
		}
	},
});

// Background blur model
const backgroundBlurModel = computed({
	get: () => config.value.uiConfig?.backgroundBlur ?? 0,
	set: (value) => {
		if (!config.value.uiConfig) {
			config.value.uiConfig = UIConfig.fromPartial({ backgroundBlur: value });
		} else {
			config.value.uiConfig.backgroundBlur = value;
		}
	},
});

// Foreground opacity model
const foregroundOpacityModel = computed({
	get: () => config.value.uiConfig?.foregroundOpacity ?? 0.4,
	set: (value) => {
		if (!config.value.uiConfig) {
			config.value.uiConfig = UIConfig.fromPartial({ foregroundOpacity: value });
		} else {
			config.value.uiConfig.foregroundOpacity = value;
		}
		setTransparent();
	},
});

// Select background image
const selectBackgroundImage = async () => {
	try {
		const selectedPath = await open({
			multiple: false,
			title: "Select Background Image",
			filters: [
				{ name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "webp"] },
			],
		});
		if (selectedPath) {
			backgroundImageModel.value = selectedPath;
		}
	} catch (error) {
		console.error("Failed to select background image:", error);
	}
};

// Clear background image
const clearBackgroundImage = () => {
	backgroundImageModel.value = "";
};

onMounted(async () => {
	await refresh_config();
});
</script>

<style scoped>
.form-section {
  transition: all var(--transition-base);
}

.form-section:hover {
  box-shadow: var(--shadow-sm);
}
</style>
