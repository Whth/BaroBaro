<template>
  <div class="general-settings">
    <h2>General Settings</h2>
    <div class="settings-form">
      <div class="form-group">
        <label for="language" class="form-label">Language</label>
        <select id="language" v-model="settings.language" class="form-select">
          <option value="en">English</option>
          <option value="es">Spanish</option>
          <option value="fr">French</option>
          <option value="de">German</option>
          <option value="zh">Chinese</option>
        </select>
      </div>
      <div class="form-group">
        <label for="theme" class="form-label">Theme</label>
        <select id="theme" v-model="settings.theme" class="form-select">
          <option value="light">Light</option>
          <option value="dark">Dark</option>
          <option value="system">System</option>
        </select>
      </div>
      <div class="form-group">
        <label for="auto-check-updates" class="form-label">
          <input
            id="auto-check-updates"
            v-model="settings.autoCheckUpdates"
            type="checkbox"
            class="form-checkbox"
          />
          Automatically check for updates
        </label>
      </div>
      <div class="form-group">
        <label for="enable-notifications" class="form-label">
          <input
            id="enable-notifications"
            v-model="settings.enableNotifications"
            type="checkbox"
            class="form-checkbox"
          />
          Enable notifications
        </label>
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
import { Config, Level, Theme, Language } from "../../proto/config";

const { config, updateConfig } = useModManager();

const settings = ref({
  language: "en",
  theme: "system",
  autoCheckUpdates: true,
  enableNotifications: true,
});

// Map protobuf loglevel to string
const logLevelToString = (level: Level): string => {
  switch (level) {
    case Level.DEBUG:
      return "debug";
    case Level.INFO:
      return "info";
    case Level.WARN:
      return "warn";
    case Level.ERROR:
      return "error";
    default:
      return "info";
  }
};

// Map string to protobuf loglevel
const stringToLogLevel = (level: string): Level => {
  switch (level) {
    case "debug":
      return Level.DEBUG;
    case "info":
      return Level.INFO;
    case "warn":
      return Level.WARN;
    case "error":
      return Level.ERROR;
    default:
      return Level.INFO;
  }
};

const saveSettings = async () => {
  try {
    // Map theme string to Theme enum
    const themeToEnum = (theme: string): Theme => {
      switch (theme) {
        case "dark":
          return Theme.DARK;
        case "light":
          return Theme.LIGHT;
        default:
          return Theme.DARK;
      }
    };

    // Map language string to Language enum
    const languageToEnum = (lang: string): Language => {
      switch (lang) {
        case "en":
          return Language.EN;
        case "zh":
          return Language.ZH;
        default:
          return Language.EN;
      }
    };

    // Create a new config object with the updated settings
    const newConfig: Config = {
      loglevel: stringToLogLevel(settings.value.theme), // Using theme as loglevel for now
      gameHome: config.value.gameHome,
      steamcmdHome: config.value.steamcmdHome,
      steamcmdConfig: config.value.steamcmdConfig,
      uiConfig: {
        theme: themeToEnum(settings.value.theme),
        language: languageToEnum(settings.value.language),
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
    settings.value.theme = logLevelToString(config.value.loglevel);
    // Other settings would be initialized here if they were in the config
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
