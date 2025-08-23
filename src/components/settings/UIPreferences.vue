<template>
  <div class="ui-preferences">
    <n-h1>{{ $t('settings.uiPreferences') }}</n-h1>

    <n-form class="settings-form" label-placement="left" label-width="160">

      <!-- Language and Theme Settings -->
      <n-card class="form-section animate-fade-in-up">
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
          <n-color-picker v-model:value="accentColorModel"/>
          <span class="color-value">{{ accentColorModel }}</span>
        </n-form-item>
      </n-card>

      <!-- Background Customization -->
      <n-card class="form-section animate-fade-in-up">
        <n-h2>{{ $t('settings.backgroundCustomization') }}</n-h2>

        <n-form-item :label="$t('settings.backgroundImage')">
          <div class="file-upload-group">
            <n-button @click="selectBackgroundImage">
              {{ $t('settings.browseForImage') }}
            </n-button>
            <n-button
                v-if="backgroundImageModel"
                class="clear-button"
                type="error"
                @click="clearBackgroundImage"
            >
              {{ $t('settings.clear') }}
            </n-button>
            <div v-if="backgroundImageModel" class="file-info">
              Selected: {{ backgroundImageModel }}
            </div>
          </div>
        </n-form-item>

        <n-form-item :label="$t('settings.backgroundOpacity')">
          <n-slider
              v-model:value="backgroundOpacityModel"
              :max="1"
              :min="0"
              :step="0.1"
          />
          <div class="range-labels">
            <span>Transparent</span>
            <span>{{ backgroundOpacityModel }}</span>
            <span>Opaque</span>
          </div>
        </n-form-item>

        <n-form-item :label="$t('settings.backgroundBlur')">
          <n-slider
              v-model:value="backgroundBlurModel"
              :max="20"
              :min="0"
              :step="1"
          />
          <div class="range-labels">
            <span>None</span>
            <span>{{ backgroundBlurModel }}px</span>
            <span>Blurry</span>
          </div>
        </n-form-item>
      </n-card>

      <div class="form-actions">
        <n-button class="save-button animate-pulse" type="primary" @click="apply_and_save_config">
          {{ $t('settings.savePreferences') }}
        </n-button>
        <n-button @click="reset_config">
          {{ $t('settings.resetPreferences') }}
        </n-button>
      </div>
    </n-form>
  </div>
</template>

<script lang="ts" setup>
import {computed, onMounted} from "vue";
import {i18n} from "@/i18n.ts";
import {config, refresh_config, reset_config, save_config} from "@/invokes.ts";
import {open} from "@tauri-apps/plugin-dialog";
import {Language, languageToJSON, Theme, UIConfig} from "@/proto/config.ts";
import {currentTheme, theme_mapping} from "@/composables/useTheme.ts";

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

const apply_and_save_config = async () => {
  await save_config()
  i18n.global.locale.value = languageToJSON(languageModel.value);
  currentTheme.value = theme_mapping[themeModel.value];
};

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

<style scoped>
.ui-preferences {
  padding: 2rem;
  max-width: 900px;
  margin: auto;
}

.settings-form {
  margin-top: 1rem;
}

.form-section {
  margin-bottom: 2rem;
  border-radius: 8px;
  overflow: hidden;
}

.file-upload-group {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  align-items: center;
}

.file-info {
  font-size: 0.9em;
  color: #666;
  margin-top: 4px;
}

.range-labels {
  display: flex;
  justify-content: space-between;
  font-size: 0.85em;
  margin-top: 6px;
  color: #777;
}

.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 2rem;
}
</style>