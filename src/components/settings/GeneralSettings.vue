<template>
  <div class="general-settings">
    <h2>General Settings</h2>
    <div class="settings-form">
      <div class="form-group">
        <label for="loglevel" class="form-label">Log Level</label>
        <select id="loglevel" v-model="settings.loglevel" class="form-select">
          <option value="TRACE">Trace</option>
          <option value="DEBUG">Debug</option>
          <option value="INFO">Info</option>
          <option value="WARN">Warning</option>
          <option value="ERROR">Error</option>
        </select>
      </div>
      <div class="form-actions">
        <button class="save-button" @click="saveSettings">Save Settings</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useModManager } from "../../composables/useModManager";
import { type Config, Level, Theme, Language } from "../../proto/config";

const { config, updateConfig } = useModManager();

const settings = ref({
	loglevel: "INFO",
});


const saveSettings = async () => {
	try {
		// Map loglevel string to Level enum
		const loglevelToEnum = (loglevel: string): Level => {
			switch (loglevel) {
				case "TRACE":
					return Level.TRACE;
				case "DEBUG":
					return Level.DEBUG;
				case "INFO":
					return Level.INFO;
				case "WARN":
					return Level.WARN;
				case "ERROR":
					return Level.ERROR;
				default:
					return Level.INFO;
			}
		};

		// Create a new config object with the updated settings
		const newConfig: Config = {
			loglevel: loglevelToEnum(settings.value.loglevel),
			gameHome: config.value.gameHome,
			steamcmdHome: config.value.steamcmdHome,
			steamcmdConfig: config.value.steamcmdConfig,
			uiConfig: {
				theme: Theme.DARK, // Default theme
				language: Language.EN, // Default language
				accentColor: "#3b82f6", // Default accent color
				backgroundImage: "",
				backgroundOpacity: 0.8,
				backgroundBlur: 10,
			},
		};

		await updateConfig(newConfig);
		console.log("General settings saved successfully!");
	} catch (error) {
		console.error("Failed to save general settings:", error);
		// TODO: Show error message to user
	}
};

onMounted(() => {
	// Initialize settings with values from config
	if (config.value) {
		// Map Level enum to string
		const levelToString = (level: Level): string => {
			switch (level) {
				case Level.TRACE:
					return "TRACE";
				case Level.DEBUG:
					return "DEBUG";
				case Level.INFO:
					return "INFO";
				case Level.WARN:
					return "WARN";
				case Level.ERROR:
					return "ERROR";
				default:
					return "INFO";
			}
		};

		settings.value.loglevel = levelToString(config.value.loglevel);
	}
	console.log("General settings mounted");
});
</script>

<style scoped>
.general-settings h2 {
  margin: 0 0 var(--spacing-l) 0;
  color: var(--color-text-primary);
  font-size: var(--font-size-heading-2);
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
  min-height: 300px;
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

.form-checkbox {
  margin-right: var(--spacing-s);
}

.form-actions {
  padding-top: var(--spacing-m);
  border-top: 1px solid var(--color-border);
}

.save-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
  width: fit-content;
}
</style>
