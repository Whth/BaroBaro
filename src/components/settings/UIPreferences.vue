<template>
  <div class="ui-preferences">
    <Title type="page">{{ t('settings.uiPreferences') }}</Title>
    <div class="settings-form">
      <!-- Language and Theme Settings -->
      <div class="form-section animate-fade-in-up">
        <Title type="section">{{ t('settings.appearanceLanguage') }}</Title>
        <div class="form-group">
          <label class="form-label" for="language">{{ t('settings.languageLabel') }}</label>
          <select id="language" v-model="preferences.language" class="form-select">
            <option value="en">{{ t('settings.english') }}</option>
            <option value="zh">{{ t('settings.chinese') }}</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label" for="theme">{{ t('settings.themeLabel') }}</label>
          <select id="theme" v-model="preferences.theme" class="form-select">
            <option value="LIGHT">{{ t('settings.light') }}</option>
            <option value="DARK">{{ t('settings.dark') }}</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label" for="accent-color">{{ t('settings.accentColorLabel') }}</label>
          <div class="color-picker">
            <input
                id="accent-color"
                v-model="preferences.accentColor"
                class="color-input"
                type="color"
            />
            <span class="color-value">{{ preferences.accentColor }}</span>
          </div>
        </div>
      </div>

      <!-- Background Customization -->
      <div class="form-section animate-fade-in-up">
        <Title type="section">{{ t('settings.backgroundCustomization') }}</Title>
        <div class="form-group">
          <label class="form-label" for="background-image"
          >{{ t('settings.backgroundImage') }}</label
          >
          <div class="file-upload">
            <button
                class="browse-button"
                @click="selectBackgroundImage"
            >
              Browse for Image
            </button>
            <button
                class="test-bg-button"
                @click="setTestBackground"
            >
              Test Background
            </button>
            <button
                v-if="backgroundSettings.backgroundImage"
                class="clear-button"
                @click="clearBackgroundImage"
            >
              {{ t('settings.clear') }}
            </button>
            <div v-if="backgroundSettings.backgroundImage" class="file-info">
              Selected: {{ backgroundSettings.backgroundImage }}
            </div>
          </div>
        </div>
        <div class="form-group">
          <label class="form-label" for="background-opacity"
          >{{ t('settings.backgroundOpacity') }}</label
          >
          <input
              id="background-opacity"
              v-model.number="backgroundSettings.backgroundOpacity"
              class="form-range"
              max="1"
              min="0"
              step="0.1"
              type="range"
          />
          <div class="range-labels">
            <span>Transparent</span>
            <span>{{ backgroundSettings.backgroundOpacity }}</span>
            <span>Opaque</span>
          </div>
        </div>
        <div class="form-group">
          <label class="form-label" for="background-blur"
          >{{ t('settings.backgroundBlur') }}</label
          >
          <input
              id="background-blur"
              v-model.number="backgroundSettings.backgroundBlur"
              class="form-range"
              max="20"
              min="0"
              step="1"
              type="range"
          />
          <div class="range-labels">
            <span>None</span>
            <span>{{ backgroundSettings.backgroundBlur }}px</span>
            <span>Blurry</span>
          </div>
        </div>
      </div>

      <div class="form-actions">
        <button class="save-button animate-pulse" @click="savePreferences">
          {{ t('settings.savePreferences') }}
        </button>
        <button class="reset-button" @click="resetPreferences">
          {{ t('settings.resetPreferences') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import i18n from "../../i18n";
import { config, refresh_config, save_config, get_background_image } from "../../invokes";
import { open } from "@tauri-apps/plugin-dialog";
import Title from "../../components/core/Title.vue";
import { useTheme } from "../../composables/useTheme";

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

const savePreferences = async () => {
	console.log("Saving UI preferences:", preferences.value);
	console.log("Saving background settings:", backgroundSettings.value);

	try {
		// Update config with new values
		if (config.value.uiConfig) {
			// Map frontend values to backend enums
			const themeMap: { [key: string]: number } = { DARK: 0, LIGHT: 1 };
			const languageMap: { [key: string]: number } = { en: 0, zh: 1 };

			config.value.uiConfig.theme = themeMap[preferences.value.theme] || 0;
			config.value.uiConfig.language =
				languageMap[preferences.value.language] || 0;
			config.value.uiConfig.accentColor = preferences.value.accentColor;
			config.value.uiConfig.backgroundImage =
				backgroundSettings.value.backgroundImage;
			config.value.uiConfig.backgroundOpacity =
				backgroundSettings.value.backgroundOpacity;
			config.value.uiConfig.backgroundBlur = Math.round(
				backgroundSettings.value.backgroundBlur,
			);

			// Save to localStorage for immediate theme application
			const bgImage = backgroundSettings.value.backgroundImage || '';
			const bgOpacity = backgroundSettings.value.backgroundOpacity.toString();
			const bgBlur = Math.round(backgroundSettings.value.backgroundBlur).toString();

			localStorage.setItem('backgroundImage', bgImage);
			localStorage.setItem('backgroundOpacity', bgOpacity);
			localStorage.setItem('backgroundBlur', bgBlur);

			// Debug logging
			console.log('Background settings saved to localStorage:', {
				backgroundImage: bgImage,
				backgroundOpacity: bgOpacity,
				backgroundBlur: bgBlur
			});

			// Force update CSS variables immediately
			document.documentElement.style.setProperty('--background-image', bgImage ? `url(${bgImage})` : 'none');
			document.documentElement.style.setProperty('--background-opacity', bgOpacity);
			document.documentElement.style.setProperty('--background-blur', `${bgBlur}px`);

			console.log('CSS variables updated:', {
				backgroundImage: bgImage ? `url(${bgImage})` : 'none',
				backgroundOpacity: bgOpacity,
				backgroundBlur: `${bgBlur}px`
			});

			// Save to backend
			await save_config();

			// Apply theme and language immediately when saving
			applyTheme(preferences.value.theme);
			applyLanguage(preferences.value.language);

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
		}
	} catch (e) {
		console.error("Failed to save preferences to backend", e);
	}
};

const applyTheme = (theme: string) => {
  // Import and use the theme composable
  const { setTheme } = useTheme();

  // Use the proper theme mode format
  const themeMode = theme.toLowerCase() as "light" | "dark";
  setTheme(themeMode);
};

const applyLanguage = (language: string) => {
	// Change i18n locale using the setLocale method
	i18n.global.locale = language as "en" | "zh";
	// Note: Language preference is now saved to backend config
};

// Watch for theme changes but don't apply immediately - only apply on save
watch(() => preferences.value.theme, (newTheme) => {
	// Theme will be applied only when user saves preferences
	console.log('Theme selected:', newTheme);
});

const resetPreferences = async () => {
	if (confirm(t("settings.resetConfirm"))) {
		try {
			// Load backend defaults
			await refresh_config();
			const themeMap: { [key: number]: string } = { 0: "DARK", 1: "LIGHT" };
			const languageMap: { [key: number]: string } = { 0: "en", 1: "zh" };

			if (config.value.uiConfig) {
				const uiConfig = config.value.uiConfig;
				preferences.value = {
					language: languageMap[uiConfig.language] || "en",
					theme: themeMap[uiConfig.theme] || "LIGHT",
					accentColor: uiConfig.accentColor || "#0969da",
				};

				backgroundSettings.value = {
					backgroundImage: uiConfig.backgroundImage || "",
					backgroundOpacity: uiConfig.backgroundOpacity || 0.2,
					backgroundBlur: Number(uiConfig.backgroundBlur) || 5,
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
			await savePreferences();

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

			await savePreferences();
		}
	}
};

const selectBackgroundImage = async () => {
 	try {
 		// Use Tauri's open dialog to get the actual file path
 		const selectedPath = await open({
 			multiple: false,
 			title: "Select Background Image",
 			filters: [
 				{ name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "webp"] },
 			],
 		});

 		if (selectedPath) {
 			console.log("Background image path selected:", selectedPath);

 			// Store the selected path - the backend will read it from config later
 			backgroundSettings.value.backgroundImage = selectedPath;
 			console.log("Background image path stored:", selectedPath);
 		}
 	} catch (error) {
 		console.error("Failed to select background image:", error);
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

const setTestBackground = () => {
 	// Set a test gradient background
 	backgroundSettings.value.backgroundImage = "linear-gradient(135deg, #667eea 0%, #764ba2 100%)";
 	backgroundSettings.value.backgroundOpacity = 0.8;
 	backgroundSettings.value.backgroundBlur = 5;
};

// Watch for theme changes - only apply when saved
// Removed immediate application to only apply on save

// Watch for language changes - only apply when saved
// Removed immediate application to only apply on save

// Refresh preferences when component becomes visible (e.g., when navigating back to settings)
onMounted(async () => {
	// Load config from backend
	try {
		await refresh_config();

		if (config.value.uiConfig) {
			const uiConfig = config.value.uiConfig;
			const themeMap: { [key: number]: string } = { 0: "DARK", 1: "LIGHT" };
			const languageMap: { [key: number]: string } = { 0: "en", 1: "zh" };

			preferences.value.theme = themeMap[uiConfig.theme] || "LIGHT";
			preferences.value.language = languageMap[uiConfig.language] || "en";
			preferences.value.accentColor = uiConfig.accentColor || "#0969da";

			backgroundSettings.value.backgroundImage = uiConfig.backgroundImage || "";
			backgroundSettings.value.backgroundOpacity =
				uiConfig.backgroundOpacity || 0.2;
			backgroundSettings.value.backgroundBlur =
				Number(uiConfig.backgroundBlur) || 5;

			console.log("UI preferences loaded from config:", {
				theme: preferences.value.theme,
				language: preferences.value.language,
				accentColor: preferences.value.accentColor,
			});
		} else {
			console.warn("No UI config found, using default values");
		}
	} catch (e) {
		console.error("Failed to load config from backend, using defaults:", e);
	}

	console.log("UI preferences component mounted");
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
   background: rgba(255, 255, 255, 0.15);
   border: 1px solid rgba(255, 255, 255, 0.2);
   border-radius: var(--border-radius-rounded);
   padding: var(--spacing-xl);
   backdrop-filter: blur(20px);
   -webkit-backdrop-filter: blur(20px);
   box-shadow:
     0 8px 32px rgba(31, 38, 135, 0.15),
     inset 0 1px 0 rgba(255, 255, 255, 0.2);
   transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
   animation-delay: 0.2s;
}

[data-theme="dark"] .form-section {
   background: rgba(31, 41, 55, 0.4);
   border-color: rgba(255, 255, 255, 0.15);
   box-shadow:
     0 8px 32px rgba(0, 0, 0, 0.3),
     inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

.form-section:nth-child(2) {
  animation-delay: 0.4s;
}

.form-section:hover {
   border-color: rgba(255, 255, 255, 0.35);
   box-shadow:
     0 12px 40px rgba(31, 38, 135, 0.2),
     inset 0 1px 0 rgba(255, 255, 255, 0.3);
   background: rgba(255, 255, 255, 0.2);
   transform: translateY(-2px);
}

[data-theme="dark"] .form-section:hover {
   border-color: rgba(255, 255, 255, 0.25);
   box-shadow:
     0 12px 40px rgba(0, 0, 0, 0.4),
     inset 0 1px 0 rgba(255, 255, 255, 0.15);
   background: rgba(31, 41, 55, 0.5);
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
  flex-wrap: wrap;
}

.file-input {
  flex: 1;
}

.browse-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
  font-size: var(--font-size-body-regular);
}

.browse-button:hover {
   background-color: var(--color-primary-dark);
}

.test-bg-button {
   padding: var(--spacing-s) var(--spacing-m);
   background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
   color: white;
   border: none;
   border-radius: var(--border-radius-soft);
   cursor: pointer;
   font-weight: var(--font-weight-medium);
   font-size: var(--font-size-body-regular);
   margin-left: var(--spacing-s);
   transition: all 0.3s ease;
}

.test-bg-button:hover {
   transform: translateY(-2px);
   box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
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

.clear-button:hover {
  background-color: var(--color-background);
  border-color: var(--color-text-secondary);
}

.file-info {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
  font-family: var(--font-family-monospace);
  padding: var(--spacing-xs) var(--spacing-s);
  background-color: var(--color-surface);
  border-radius: var(--border-radius-soft);
  border: 1px solid var(--color-border);
  max-width: 300px;
  word-break: break-all;
}
</style>
