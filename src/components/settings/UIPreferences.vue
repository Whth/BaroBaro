<template>
  <div class="ui-preferences">
    <n-h1>{{ $t('settings.uiPreferences') }}</n-h1>

    <n-form class="settings-form" label-placement="left" label-width="160">

      <!-- Language and Theme Settings -->
      <n-card class="form-section animate-fade-in-up">
        <n-h2>{{ $t('settings.appearanceLanguage') }}</n-h2>

        <n-form-item :label="$t('settings.languageLabel')">
          <n-select
              v-model:value="lang"
              :options="[
                { label: lang_show_mapping[Language.EN], value: Language.EN },
                { label: lang_show_mapping[Language.ZH], value: Language.ZH }
              ]"
          />
        </n-form-item>

        <n-form-item :label="$t('settings.themeLabel')">
          <n-select
              v-model:value="theme"
              :options="
              [
                { label: $t('settings.light'), value: Theme.Light },
                { label: $t('settings.dark'), value: Theme.Dark }
              ]"
          />
        </n-form-item>

        <n-form-item :label="$t('settings.accentColorLabel')">
          <n-color-picker v-model:value="config.uiConfig.accentColor"/>
          <span class="color-value">{{ config.uiConfig?.accentColor }}</span>
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
                v-if="config.uiConfig.backgroundImage"
                class="clear-button"
                type="error"
                @click="clearBackgroundImage"
            >
              {{ $t('settings.clear') }}
            </n-button>
            <div v-if="config.uiConfig.backgroundImage" class="file-info">
              Selected: {{ config.uiConfig?.backgroundImage }}
            </div>
          </div>
        </n-form-item>

        <n-form-item :label="$t('settings.backgroundOpacity')">
          <n-slider
              v-model:value="config.uiConfig.backgroundOpacity"
              :max="1"
              :min="0"
              :step="0.1"
          />
          <div class="range-labels">
            <span>Transparent</span>
            <span>{{ config.uiConfig?.backgroundOpacity }}</span>
            <span>Opaque</span>
          </div>
        </n-form-item>

        <n-form-item :label="$t('settings.backgroundBlur')">
          <n-slider
              v-model:value="config.uiConfig.backgroundBlur"
              :max="20"
              :min="0"
              :step="1"
          />
          <div class="range-labels">
            <span>None</span>
            <span>{{ config.uiConfig?.backgroundBlur }}px</span>
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
import {onMounted, ref, Ref, watch} from "vue";
import {i18n} from "../../i18n";
import {config, refresh_config, reset_config, save_config,} from "../../invokes";
import {open} from "@tauri-apps/plugin-dialog";
import {Language, languageToJSON, Theme, UIConfig} from "../../proto/config.ts";
import {currentTheme, theme_mapping} from "../../composables/useTheme.ts";


const lang_show_mapping = {
  [Language.EN]: "English",
  [Language.ZH]: "中文",
  [Language.UNRECOGNIZED]: "Unknown"
}


const lang: Ref<Language> = ref(Language.EN);
const theme: Ref<Theme> = ref(Theme.Light);

watch(lang, (value, oldValue) => {
  console.log(`lang changing ${oldValue} => ${value}`)
  if (value != oldValue) {
    if (config.value.uiConfig !== undefined) {
      console.log("Change language to " + value)
      config.value.uiConfig.language = value;
    } else {
      config.value.uiConfig = UIConfig.fromPartial({language: value});
    }
  }
})
watch(theme, (value, oldValue) => {
  if (value != oldValue) {
    if (config.value.uiConfig !== undefined) {
      console.log("Change theme to " + value)
      config.value.uiConfig.theme = value;
    } else {
      config.value.uiConfig = UIConfig.fromPartial({theme: value});
    }
  }
})

const apply_and_save_config = async () => {
  await save_config()
  i18n.global.locale.value = languageToJSON(config.value.uiConfig?.language ?? Language.ZH);
  currentTheme.value = theme_mapping[config.value.uiConfig?.theme ?? Theme.Light];
};


const selectBackgroundImage = async () => {
  try {
    // Use Tauri's open dialog to get the actual file path
    const selectedPath = await open({
      multiple: false,
      title: "Select Background Image",
      filters: [
        {name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "webp"]},
      ],
    });

    if (selectedPath) {
      console.log("Background image path selected:", selectedPath);

      // Store the selected path - the backend will read it from config later

      if (config.value.uiConfig !== undefined) {
        config.value.uiConfig.backgroundImage = selectedPath;
      } else {
        config.value.uiConfig = UIConfig.fromPartial({backgroundImage: selectedPath});
      }
      console.log("Background image path stored:", selectedPath);
    }
  } catch (error) {
    console.error("Failed to select background image:", error);
  }
};

const clearBackgroundImage = () => {
  if (config.value.uiConfig !== undefined) {
    config.value.uiConfig.backgroundImage = "";
  }
};
onMounted(async () => {
  await refresh_config();
  if (config.value.uiConfig !== undefined) {
    lang.value = config.value.uiConfig.language;
    theme.value = config.value.uiConfig.theme;
  }
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