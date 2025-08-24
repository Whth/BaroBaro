<template>
  <n-card class="settings-card">
    <n-form class="settings-form">
      <n-form-item :label="$t('settings.logLevel')">
        <n-select
            v-model:value="loglevel"
            :options="logLevelOptions"
            style="width: 200px"
        />
      </n-form-item>

      <n-form-item :label="$t('settings.gameHome')">
        <n-input-group>
          <n-input
              v-model:value="settings.gamePath"
              :placeholder="$t('settings.gameHome')"
              readonly
          />
          <n-button @click="browseGamePath">{{ $t('settings.browse') }}</n-button>
        </n-input-group>
      </n-form-item>

      <n-form-item :label="$t('settings.steamCmdHome')">
        <n-input-group>
          <n-input
              v-model:value="settings.steamcmdPath"
              :placeholder="$t('settings.steamCmdHome')"
              readonly
          />
          <n-button @click="browseSteamCmdPath">{{ $t('settings.browse') }}</n-button>
        </n-input-group>
      </n-form-item>

      <n-form-item :label="$t('settings.steamUsername')">
        <n-input
            v-model:value="settings.steamcmdUsername"
            :placeholder="$t('settings.steamUsername')"
        />
      </n-form-item>

      <n-form-item :label="$t('settings.steamPassword')">
        <n-input
            v-model:value="settings.steamcmdPassword"
            :placeholder="$t('settings.steamPassword')"
            show-password-on="click"
            type="password"
        />
      </n-form-item>

      <n-form-item :label="$t('settings.parallelDownloads')">
        <n-input-number
            v-model:value="settings.steamcmdParallel"
            :max="10"
            :min="1"
            :placeholder="$t('settings.parallelDownloads')"
        />
      </n-form-item>

    </n-form>
  </n-card>
</template>

<script lang="ts" setup>
import { config, refresh_config, save_config } from "../../invokes.ts";
import { Level } from "../../proto/config.ts";
import { computed, onMounted, ref } from "vue";
import { useModManager } from "../../composables/useModManager";
import { message, open } from "@tauri-apps/plugin-dialog";

const {
	config: modConfig,
	updateGameHome,
	updateSteamCmdHome,
} = useModManager();

const loglevel = computed({
	get: () => config.value.loglevel,
	set: (newValue) => {
		if (newValue === Level.UNRECOGNIZED) {
			newValue = Level.Info;
		}
		config.value.loglevel = newValue;
	},
});

const logLevelOptions = [
	{ label: "Trace", value: Level.Trace },
	{ label: "Debug", value: Level.Debug },
	{ label: "Info", value: Level.Info },
	{ label: "Warning", value: Level.Warn },
	{ label: "Error", value: Level.Error },
];

const settings = ref({
	gamePath: "",
	steamcmdPath: "",
	steamcmdUsername: "",
	steamcmdPassword: "",
	steamcmdParallel: 1,
});

const showError = async (title: string, error: any) => {
	console.error(title, error);
	await message(`Error: ${error.message || error}`, { title, kind: "error" });
};

const browseGamePath = async () => {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select Barotrauma Game Installation Path",
		});

		if (selected) {
			settings.value.gamePath = selected as string;
			await updateGameHome(selected as string);
		}
	} catch (error) {
		await showError("Failed to select game path", error);
	}
};

const browseSteamCmdPath = async () => {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select SteamCMD Path",
		});

		if (selected) {
			settings.value.steamcmdPath = selected as string;
			await updateSteamCmdHome(selected as string);
		}
	} catch (error) {
		await showError("Failed to select SteamCMD path", error);
	}
};

const saveAllSettings = async () => {
	try {
		// Save general config
		await save_config();

		// Save paths settings
		if (settings.value.gamePath) await updateGameHome(settings.value.gamePath);
		if (settings.value.steamcmdPath)
			await updateSteamCmdHome(settings.value.steamcmdPath);

		// TODO: Save SteamCMD config if needed
		console.log("All settings to save:", settings.value);

		await message("All settings saved successfully!", {
			title: "Success",
			kind: "info",
		});
	} catch (error) {
		await showError("Failed to save settings", error);
	}
};

onMounted(() => {
	refresh_config();
	if (modConfig.value) {
		settings.value.gamePath = modConfig.value.gameHome;
		settings.value.steamcmdPath = modConfig.value.steamcmdHome;
		if (modConfig.value.steamcmdConfig) {
			settings.value.steamcmdUsername = modConfig.value.steamcmdConfig.username;
			settings.value.steamcmdPassword = modConfig.value.steamcmdConfig.password;
			settings.value.steamcmdParallel = modConfig.value.steamcmdConfig.parallel;
		}
	}
});
</script>

