<template>
  <n-grid :cols="1" :x-gap="16" :y-gap="16">
    <n-gi>

      <!-- Language and Theme Settings -->
      <n-card class="form-section animate-fade-in">
        <n-h2>{{ $t('settings.appearanceLanguage') }}</n-h2>

        <n-form-item :label="$t('settings.languageLabel')">
          <n-select
              v-model:value="languageModel"
              :options="[
                { label: lang_show_mapping[Language.EN], value: Language.EN },
                { label: lang_show_mapping[Language.ZH], value: Language.ZH }
              ]"
          />
        </n-form-item>

        <n-form-item :label="$t('settings.themeLabel')">
          <n-select
              v-model:value="themeModel"
              :options="
              [
                { label: $t('settings.light'), value: Theme.Light },
                { label: $t('settings.dark'), value: Theme.Dark }
              ]"
          />
        </n-form-item>

        <n-form-item :label="$t('settings.accentColorLabel')">
          <n-color-picker v-model:value="accentColorModel" size="small"/>
        </n-form-item>
      </n-card>
    </n-gi>


    <n-gi>
      <!-- Background Customization -->
      <n-card class="form-section animate-fade-in">
        <n-h2>{{ $t('settings.backgroundCustomization') }}</n-h2>

        <n-form-item :label="$t('settings.backgroundImage')">
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

        <n-form-item :label="$t('settings.backgroundOpacity')">
          <n-slider
              v-model:value="backgroundOpacityModel"
              :max="1"
              :min="0"
              :step="0.1"
          />
        </n-form-item>

        <n-form-item :label="$t('settings.backgroundBlur')">
          <n-slider
              v-model:value="backgroundBlurModel"
              :max="20"
              :min="0"
              :step="1"
          />
        </n-form-item>
      </n-card>

    </n-gi>
  </n-grid>
</template>

<script lang="ts" setup>
import {computed, onMounted} from "vue";
import {config, refresh_config} from "@/invokes.ts";
import {open} from "@tauri-apps/plugin-dialog";
import {Language, Theme, UIConfig} from "@/proto/config.ts";

const lang_show_mapping = {
  [Language.EN]: "English",
  [Language.ZH]: "中文",
  [Language.UNRECOGNIZED]: "Unknown"
}

// 计算属性处理所有可能为 undefined 的字段
const languageModel = computed({
  get: () => config.value.uiConfig?.language ?? Language.ZH,
  set: (value) => {
    if (!config.value.uiConfig) {
      config.value.uiConfig = UIConfig.fromPartial({language: value});
    } else {
      config.value.uiConfig.language = value;
    }
  }
})

const themeModel = computed({
  get: () => config.value.uiConfig?.theme ?? Theme.Light,
  set: (value) => {
    if (!config.value.uiConfig) {
      config.value.uiConfig = UIConfig.fromPartial({theme: value});
    } else {
      config.value.uiConfig.theme = value;
    }
  }
})

const accentColorModel = computed({
  get: () => config.value.uiConfig?.accentColor ?? '#18a058',
  set: (value) => {
    if (!config.value.uiConfig) {
      config.value.uiConfig = UIConfig.fromPartial({accentColor: value});
    } else {
      config.value.uiConfig.accentColor = value;
    }
  }
})

const backgroundImageModel = computed({
  get: () => config.value.uiConfig?.backgroundImage ?? '',
  set: (value) => {
    if (!config.value.uiConfig) {
      config.value.uiConfig = UIConfig.fromPartial({backgroundImage: value});
    } else {
      config.value.uiConfig.backgroundImage = value;
    }
  }
})

const backgroundOpacityModel = computed({
  get: () => config.value.uiConfig?.backgroundOpacity ?? 1,
  set: (value) => {
    if (!config.value.uiConfig) {
      config.value.uiConfig = UIConfig.fromPartial({backgroundOpacity: value});
    } else {
      config.value.uiConfig.backgroundOpacity = value;
    }
  }
})

const backgroundBlurModel = computed({
  get: () => config.value.uiConfig?.backgroundBlur ?? 0,
  set: (value) => {
    if (!config.value.uiConfig) {
      config.value.uiConfig = UIConfig.fromPartial({backgroundBlur: value});
    } else {
      config.value.uiConfig.backgroundBlur = value;
    }
  }
})


const selectBackgroundImage = async () => {
  try {
    const selectedPath = await open({
      multiple: false,
      title: "Select Background Image",
      filters: [
        {name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "webp"]},
      ],
    });

    if (selectedPath) {
      backgroundImageModel.value = selectedPath;
    }
  } catch (error) {
    console.error("Failed to select background image:", error);
  }
};

const clearBackgroundImage = () => {
  backgroundImageModel.value = '';
};

onMounted(async () => {
  await refresh_config();
});
</script>
