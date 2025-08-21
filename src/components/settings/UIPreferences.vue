<template>
  <div class="ui-preferences">
    <h2>UI Preferences</h2>
    <div class="settings-form">
      <div class="form-group">
        <label for="accent-color" class="form-label">Accent Color</label>
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
      <div class="form-group">
        <label for="layout-density" class="form-label">Layout Density</label>
        <select
          id="layout-density"
          v-model="preferences.layoutDensity"
          class="form-select"
        >
          <option value="compact">Compact</option>
          <option value="normal">Normal</option>
          <option value="spacious">Spacious</option>
        </select>
      </div>
      <div class="form-group">
        <label class="form-label">Sidebar Position</label>
        <div class="radio-group">
          <label class="radio-option">
            <input
              v-model="preferences.sidebarPosition"
              type="radio"
              value="left"
              class="form-radio"
            />
            Left
          </label>
          <label class="radio-option">
            <input
              v-model="preferences.sidebarPosition"
              type="radio"
              value="right"
              class="form-radio"
            />
            Right
          </label>
        </div>
      </div>
      <div class="form-group">
        <label for="animation-speed" class="form-label">Animation Speed</label>
        <input
          id="animation-speed"
          v-model.number="preferences.animationSpeed"
          type="range"
          min="0"
          max="2"
          step="0.1"
          class="form-range"
        />
        <div class="range-labels">
          <span>Slow</span>
          <span>Normal</span>
          <span>Fast</span>
        </div>
      </div>
      <div class="form-group">
        <label for="show-tooltips" class="form-label">
          <input
            id="show-tooltips"
            v-model="preferences.showTooltips"
            type="checkbox"
            class="form-checkbox"
          />
          Show tooltips
        </label>
      </div>

      <!-- Background Customization -->
      <div class="form-section">
        <h3>Background Customization</h3>
        <div class="form-group">
          <label for="background-image" class="form-label"
            >Background Image</label
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
              Clear
            </button>
          </div>
        </div>
        <div class="form-group">
          <label for="background-opacity" class="form-label"
            >Background Opacity</label
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
            >Background Blur</label
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
          Save Preferences
        </button>
        <button class="reset-button" @click="resetPreferences">
          Reset to Defaults
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";

interface UIPreferences {
	accentColor: string;
	layoutDensity: string;
	sidebarPosition: string;
	animationSpeed: number;
	showTooltips: boolean;
}

interface BackgroundSettings {
	backgroundImage: string;
	backgroundOpacity: number;
	backgroundBlur: number;
}

const preferences = ref<UIPreferences>({
	accentColor: "#3B82F6",
	layoutDensity: "normal",
	sidebarPosition: "left",
	animationSpeed: 1,
	showTooltips: true,
});

const backgroundSettings = ref<BackgroundSettings>({
	backgroundImage: "",
	backgroundOpacity: 1,
	backgroundBlur: 0,
});

const savePreferences = () => {
	console.log("Saving UI preferences:", preferences.value);
	console.log("Saving background settings:", backgroundSettings.value);

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

const _resetPreferences = () => {
	if (
		confirm("Are you sure you want to reset all UI preferences to defaults?")
	) {
		preferences.value = {
			accentColor: "#3B82F6",
			layoutDensity: "normal",
			sidebarPosition: "left",
			animationSpeed: 1,
			showTooltips: true,
		};

		backgroundSettings.value = {
			backgroundImage: "",
			backgroundOpacity: 1,
			backgroundBlur: 0,
		};

		// Save the reset preferences
		savePreferences();

		console.log("UI preferences reset to defaults");
	}
};

const _handleBackgroundImageUpload = (event: Event) => {
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

const _clearBackgroundImage = () => {
	backgroundSettings.value.backgroundImage = "";
	// Clear the file input
	const fileInput = document.getElementById(
		"background-image",
	) as HTMLInputElement;
	if (fileInput) {
		fileInput.value = "";
	}
};

onMounted(() => {
	// Load preferences from localStorage
	const savedPreferences = localStorage.getItem("uiPreferences");
	if (savedPreferences) {
		try {
			preferences.value = JSON.parse(savedPreferences);
		} catch (e) {
			console.error("Failed to parse UI preferences", e);
		}
	}

	// Load background settings from localStorage
	const savedBackgroundSettings = localStorage.getItem("backgroundSettings");
	if (savedBackgroundSettings) {
		try {
			backgroundSettings.value = JSON.parse(savedBackgroundSettings);
		} catch (e) {
			console.error("Failed to parse background settings", e);
		}
	}

	console.log("UI preferences mounted");
});
</script>

<style scoped>
.ui-preferences h2 {
  margin: 0 0 var(--spacing-l) 0;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-label {
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.color-picker {
  display: flex;
  align-items: center;
  gap: var(--spacing-m);
}

.color-input {
  width: 50px;
  height: 30px;
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
}

.color-value {
  font-family: var(--font-family-monospace);
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.form-select {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
  width: 100%;
  max-width: 300px;
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
  padding-top: var(--spacing-m);
  border-top: 1px solid var(--color-border);
}

.save-button,
.reset-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.save-button {
  background-color: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.reset-button {
  background-color: var(--color-surface);
  color: var(--color-text-primary);
}

.form-section {
  padding: var(--spacing-m) 0;
  border-top: 1px solid var(--color-border);
}

.form-section h3 {
  margin: 0 0 var(--spacing-m) 0;
  color: var(--color-text-primary);
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
