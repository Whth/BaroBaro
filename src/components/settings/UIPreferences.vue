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
            <button
              class="browse-button"
              @click="selectBackgroundImage"
            >
              Browse for Image
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
import { useI18n } from "vue-i18n";
import i18n from "../../i18n";
import { config, refresh_config, save_config } from "../../invokes";
import { open } from "@tauri-apps/plugin-dialog";

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

			config.value.uiConfig.theme = themeMap[preferences.value.theme] || 1;
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

			// Save to backend
			await save_config();

			// Apply theme and language immediately
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
	const root = document.documentElement;

	// Add transition class for smooth theme change
	root.style.setProperty("--theme-transition-duration", "300ms");
	root.classList.add("theme-transitioning");

	// Set the theme
	root.setAttribute("data-theme", theme.toLowerCase());

	// Remove transition class after animation completes
	setTimeout(() => {
		root.classList.remove("theme-transitioning");
		root.style.removeProperty("--theme-transition-duration");
	}, 300);
};

const applyLanguage = (language: string) => {
	// Change i18n locale using the setLocale method
	i18n.global.locale = language as "en" | "zh";
	// Note: Language preference is now saved to backend config
};

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
      filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'] }]
    });

    if (selectedPath) {
      // Save the full file path instead of just the name
      backgroundSettings.value.backgroundImage = selectedPath;
      console.log("Background image path selected:", selectedPath);
    }
  } catch (error) {
    console.error("Failed to select background image:", error);
  }
};

const handleBackgroundImageUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];

  if (file) {
    try {
      // Fallback: save the file name if dialog approach fails
      backgroundSettings.value.backgroundImage = file.name;
      console.log("Background image file selected (fallback):", file.name);
    } catch (error) {
      console.error("Failed to handle background image upload:", error);
    }
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


// Watch for theme changes - only apply when saved
// Removed immediate application to only apply on save

// Watch for language changes - only apply when saved
// Removed immediate application to only apply on save

// Method to refresh preferences from config
const refreshPreferences = async () => {
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
			backgroundSettings.value.backgroundOpacity = uiConfig.backgroundOpacity || 0.2;
			backgroundSettings.value.backgroundBlur = Number(uiConfig.backgroundBlur) || 5;

			console.log("Preferences refreshed from config");
		}
	} catch (e) {
		console.error("Failed to refresh preferences:", e);
	}
};

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
			backgroundSettings.value.backgroundOpacity = uiConfig.backgroundOpacity || 0.2;
			backgroundSettings.value.backgroundBlur = Number(uiConfig.backgroundBlur) || 5;

			console.log("UI preferences loaded from config:", {
				theme: preferences.value.theme,
				language: preferences.value.language,
				accentColor: preferences.value.accentColor
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
