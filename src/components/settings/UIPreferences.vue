<template>
  <div class="ui-preferences">
    <h2>{{ t('settings.uiPreferences') }}</h2>
    <div class="settings-form">
      <!-- Language and Theme Settings -->
      <div class="form-section">
        <h3>{{ t('settings.appearanceLanguage') }}</h3>
        <div class="form-group">
          <label for="language" class="form-label">{{ t('settings.languageLabel') }}</label>
          <select id="language" v-model="preferences.language" class="form-select">
            <option value="en">{{ t('settings.english') }}</option>
            <option value="zh">{{ t('settings.chinese') }}</option>
          </select>
        </div>
        <div class="form-group">
          <label for="theme" class="form-label">{{ t('settings.themeLabel') }}</label>
          <select id="theme" v-model="preferences.theme" class="form-select">
            <option value="LIGHT">{{ t('settings.light') }}</option>
            <option value="DARK">{{ t('settings.dark') }}</option>
          </select>
        </div>
        <div class="form-group">
          <label for="accent-color" class="form-label">{{ t('settings.accentColorLabel') }}</label>
          <div class="color-picker">
            <input
              id="accent-color"
              v-model="preferences.accentColor"
              type="color"
              class="color-input"
            />
            <span class="color-value">{{ preferences.accentColor }}</span>
          </div>
        </div>
      </div>

      <!-- Background Customization -->
      <div class="form-section">
        <h3>{{ t('settings.backgroundCustomization') }}</h3>
        <div class="form-group">
          <label for="background-image" class="form-label"
            >{{ t('settings.backgroundImage') }}</label
          >
          <div class="file-upload">
            <input
              id="background-image"
              type="file"
              accept="image/*"
              class="file-input"
              @change="handleBackgroundImageUpload"
            />
            <button
              v-if="backgroundSettings.backgroundImage"
              class="clear-button"
              @click="clearBackgroundImage"
            >
              {{ t('settings.clear') }}
            </button>
          </div>
        </div>
        <div class="form-group">
          <label for="background-opacity" class="form-label"
            >{{ t('settings.backgroundOpacity') }}</label
          >
          <input
            id="background-opacity"
            v-model.number="backgroundSettings.backgroundOpacity"
            type="range"
            min="0"
            max="1"
            step="0.1"
            class="form-range"
          />
          <div class="range-labels">
            <span>Transparent</span>
            <span>{{ backgroundSettings.backgroundOpacity }}</span>
            <span>Opaque</span>
          </div>
        </div>
        <div class="form-group">
          <label for="background-blur" class="form-label"
            >{{ t('settings.backgroundBlur') }}</label
          >
          <input
            id="background-blur"
            v-model.number="backgroundSettings.backgroundBlur"
            type="range"
            min="0"
            max="20"
            step="1"
            class="form-range"
          />
          <div class="range-labels">
            <span>None</span>
            <span>{{ backgroundSettings.backgroundBlur }}px</span>
            <span>Blurry</span>
          </div>
        </div>
      </div>

      <div class="form-actions">
        <button class="save-button" @click="savePreferences">
          {{ t('settings.savePreferences') }}
        </button>
        <button class="reset-button" @click="resetPreferences">
          {{ t('settings.resetPreferences') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import i18n from '../../i18n';

const { t } = useI18n();

interface UIPreferences {
	language: string;
	theme: string;
	accentColor: string;
}

interface BackgroundSettings {
	backgroundImage: string;
	backgroundOpacity: number;
	backgroundBlur: number;
}

const preferences = ref<UIPreferences>({
	language: "en",
	theme: "LIGHT",
	accentColor: "#0969da",
});

const backgroundSettings = ref<BackgroundSettings>({
	backgroundImage: "",
	backgroundOpacity: 1,
	backgroundBlur: 0,
});

const savePreferences = () => {
	console.log("Saving UI preferences:", preferences.value);
	console.log("Saving background settings:", backgroundSettings.value);

	// Apply theme immediately
	applyTheme(preferences.value.theme);

	// Apply language immediately
	applyLanguage(preferences.value.language);

	// Save to localStorage
	localStorage.setItem("uiPreferences", JSON.stringify(preferences.value));
	localStorage.setItem(
		"backgroundSettings",
		JSON.stringify(backgroundSettings.value),
	);

	// Emit an event to notify other components of the changes
	window.dispatchEvent(
		new CustomEvent("ui-preferences-updated", {
			detail: {
				preferences: preferences.value,
				backgroundSettings: backgroundSettings.value,
			},
		}),
	);

	console.log("UI preferences and background settings saved successfully!");
};

const applyTheme = (theme: string) => {
	const root = document.documentElement;

	// Add transition class for smooth theme change
	root.style.setProperty('--theme-transition-duration', '300ms');
	root.classList.add('theme-transitioning');

	// Set the theme
	root.setAttribute('data-theme', theme.toLowerCase());

	// Remove transition class after animation completes
	setTimeout(() => {
		root.classList.remove('theme-transitioning');
		root.style.removeProperty('--theme-transition-duration');
	}, 300);
};

const applyLanguage = (language: string) => {
	// Change i18n locale using the setLocale method
	i18n.global.locale = language as 'en' | 'zh';
	// Save language preference to localStorage for persistence across sessions
	localStorage.setItem("language", language);
};

const resetPreferences = async () => {
	if (
		confirm(t('settings.resetConfirm'))
	) {
		try {
			// Get backend defaults
			const config = await invoke('read_config') as any;
			const themeMap: { [key: number]: string } = { 0: 'DARK', 1: 'LIGHT' };
			const languageMap: { [key: number]: string } = { 0: 'EN', 1: 'ZH' };

			if (config.ui_config) {
				const uiConfig = config.ui_config;
				preferences.value = {
					language: languageMap[uiConfig.language] || "EN",
					theme: themeMap[uiConfig.theme] || "LIGHT",
					accentColor: uiConfig.accent_color || "#0969da",
				};

				backgroundSettings.value = {
					backgroundImage: uiConfig.background_image || "",
					backgroundOpacity: uiConfig.background_opacity || 0.2,
					backgroundBlur: uiConfig.background_blur || 5,
				};
			} else {
				// Fallback to hardcoded defaults if no config
				preferences.value = {
					language: "en",
					theme: "LIGHT",
					accentColor: "#0969da",
				};

				backgroundSettings.value = {
					backgroundImage: "",
					backgroundOpacity: 0.2,
					backgroundBlur: 5,
				};
			}

			// Save the reset preferences
			savePreferences();

			console.log("UI preferences reset to defaults");
		} catch (e) {
			console.error("Failed to load backend defaults for reset", e);
			// Fallback to hardcoded defaults
			preferences.value = {
				language: "en",
				theme: "LIGHT",
				accentColor: "#0969da",
			};

			backgroundSettings.value = {
				backgroundImage: "",
				backgroundOpacity: 0.2,
				backgroundBlur: 5,
			};

			savePreferences();
		}
	}
};

const handleBackgroundImageUpload = (event: Event) => {
	const target = event.target as HTMLInputElement;
	const file = target.files?.[0];

	if (file) {
		const reader = new FileReader();
		reader.onload = (e) => {
			backgroundSettings.value.backgroundImage = e.target?.result as string;
		};
		reader.readAsDataURL(file);
	}
};

const clearBackgroundImage = () => {
	backgroundSettings.value.backgroundImage = "";
	// Clear the file input
	const fileInput = document.getElementById(
		"background-image",
	) as HTMLInputElement;
	if (fileInput) {
		fileInput.value = "";
	}
};

onMounted(async () => {
	// Load config from backend first
	try {
		const config = await invoke('read_config') as any;
		if (config.ui_config) {
			const uiConfig = config.ui_config;
			// Map backend theme enum to frontend string
			const themeMap: { [key: number]: string } = { 0: 'DARK', 1: 'LIGHT' };
			const languageMap: { [key: number]: string } = { 0: 'en', 1: 'zh' };

			preferences.value.theme = themeMap[uiConfig.theme] || "LIGHT";
			preferences.value.language = languageMap[uiConfig.language] || "EN";
			preferences.value.accentColor = uiConfig.accent_color || "#0969da";

			// Update background settings
			backgroundSettings.value.backgroundImage = uiConfig.background_image || "";
			backgroundSettings.value.backgroundOpacity = uiConfig.background_opacity || 0.2;
			backgroundSettings.value.backgroundBlur = uiConfig.background_blur || 5;
		}
	} catch (e) {
		console.error("Failed to load config from backend", e);
	}

	// Load preferences from localStorage (overrides backend defaults)
	const savedPreferences = localStorage.getItem("uiPreferences");
	if (savedPreferences) {
		try {
			const parsed = JSON.parse(savedPreferences);
			preferences.value = {
				language: parsed.language || preferences.value.language,
				theme: parsed.theme || preferences.value.theme,
				accentColor: parsed.accentColor || preferences.value.accentColor,
			};
		} catch (e) {
			console.error("Failed to parse UI preferences", e);
		}
	}

	// Apply the initial language setting
	applyLanguage(preferences.value.language);

	// Load background settings from localStorage
	const savedBackgroundSettings = localStorage.getItem("backgroundSettings");
	if (savedBackgroundSettings) {
		try {
			backgroundSettings.value = JSON.parse(savedBackgroundSettings);
		} catch (e) {
			console.error("Failed to parse background settings", e);
		}
	}

	// Apply the theme
	applyTheme(preferences.value.theme);

	console.log("UI preferences mounted");
});

// Watch for theme changes and apply them immediately
watch(() => preferences.value.theme, (newTheme) => {
	if (newTheme) {
		applyTheme(newTheme);
	}
});

// Watch for language changes and apply them immediately
watch(() => preferences.value.language, (newLanguage) => {
	if (newLanguage) {
		applyLanguage(newLanguage);
	}
});
</script>

<style scoped>
.ui-preferences h2 {
   margin: 0 0 var(--spacing-xl) 0;
   color: var(--color-text-primary);
   font-size: var(--font-size-heading-2);
   font-weight: 600;
   letter-spacing: -0.025em;
}

.settings-form {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-xl);
   min-height: 400px;
}

.form-group {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-s);
   margin-bottom: var(--spacing-l);
}

.form-label {
   font-weight: 500;
   color: var(--color-text-primary);
   font-size: var(--font-size-body-regular);
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.form-label::after {
   content: ":";
   opacity: 0.6;
}

.color-picker {
   display: flex;
   align-items: center;
   gap: var(--spacing-m);
   padding: var(--spacing-s);
   background: var(--color-background);
   border-radius: var(--border-radius-soft);
   border: 1px solid var(--color-border);
}

.color-input {
   width: 60px;
   height: 40px;
   border: 2px solid var(--color-border);
   border-radius: var(--border-radius-soft);
   cursor: pointer;
   transition: all 0.2s ease;
}

.color-input:hover {
   border-color: var(--color-primary);
   transform: scale(1.05);
}

.color-value {
   font-family: var(--font-family-monospace);
   font-size: var(--font-size-body-small);
   color: var(--color-text-secondary);
   font-weight: 500;
   background: var(--color-surface);
   padding: var(--spacing-xs) var(--spacing-s);
   border-radius: var(--border-radius-soft);
   border: 1px solid var(--color-border);
}

.form-select {
   padding: var(--spacing-s) var(--spacing-m);
   border: 2px solid var(--color-border);
   border-radius: var(--border-radius-soft);
   background-color: var(--color-background);
   color: var(--color-text-primary);
   font-size: var(--font-size-body-regular);
   transition: all 0.2s ease;
   cursor: pointer;
   width: 100%;
   max-width: 300px;
}

.form-select:hover {
   border-color: var(--color-primary);
}

.form-select:focus {
   outline: none;
   border-color: var(--color-primary);
   box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.radio-group {
  display: flex;
  gap: var(--spacing-m);
}

.radio-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-weight: var(--font-weight-regular);
  color: var(--color-text-primary);
}

.form-radio {
  margin-right: var(--spacing-xs);
}

.form-range {
  width: 100%;
  max-width: 300px;
}

.range-labels {
  display: flex;
  justify-content: space-between;
  width: 100%;
  max-width: 300px;
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.form-checkbox {
  margin-right: var(--spacing-s);
}

.form-actions {
   display: flex;
   gap: var(--spacing-m);
   padding-top: var(--spacing-xl);
   border-top: 1px solid var(--color-border);
   margin-top: var(--spacing-xl);
   justify-content: flex-end;
}

.save-button,
.reset-button {
   padding: var(--spacing-m) var(--spacing-xl);
   border: 2px solid var(--color-border);
   border-radius: var(--border-radius-soft);
   cursor: pointer;
   font-weight: 600;
   font-size: var(--font-size-body-regular);
   transition: all 0.2s ease;
   min-width: 120px;
   height: 44px;
   display: flex;
   align-items: center;
   justify-content: center;
}

.save-button {
   background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-dark) 100%);
   color: white;
   border-color: var(--color-primary);
   box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
}

.save-button:hover {
   transform: translateY(-2px);
   box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
}

.reset-button {
   background: var(--color-surface);
   color: var(--color-text-primary);
   border-color: var(--color-border);
}

.reset-button:hover {
   background: var(--color-background);
   border-color: var(--color-text-secondary);
}

.form-section {
   background: var(--color-surface);
   border: 1px solid var(--color-border);
   border-radius: var(--border-radius-rounded);
   padding: var(--spacing-xl);
   transition: all 0.2s ease;
}

.form-section:hover {
   border-color: var(--color-primary-light);
   box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.form-section h3 {
   margin: 0 0 var(--spacing-l) 0;
   color: var(--color-text-primary);
   font-size: var(--font-size-heading-3);
   font-weight: 600;
   display: flex;
   align-items: center;
   gap: var(--spacing-s);
}

.form-section h3::before {
   content: "⚙️";
   font-size: var(--font-size-body-large);
}

.file-upload {
  display: flex;
  align-items: center;
  gap: var(--spacing-m);
}

.file-input {
  flex: 1;
}

.clear-button {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
  font-size: var(--font-size-body-small);
}
</style>
