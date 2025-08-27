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
              v-model:value="config.gameHome"
              :placeholder="$t('settings.gameHome')"
              readonly
          />
          <n-button @click="browseGamePath">{{ $t('settings.browse') }}</n-button>
        </n-input-group>
      </n-form-item>

      <n-form-item :label="$t('settings.steamCmdHome')">
        <n-input-group>
          <n-input
              v-model:value="config.steamcmdHome"
              :placeholder="$t('settings.steamCmdHome')"
              readonly
          />
          <n-button @click="browseSteamCmdPath">{{ $t('settings.browse') }}</n-button>
        </n-input-group>
      </n-form-item>


      <n-form-item :label="$t('settings.steamUsername')">
        <n-input
            v-model:value="steamcmd_uname"
            :placeholder="$t('settings.steamUsername')"
        />
      </n-form-item>

      <n-form-item :label="$t('settings.steamPassword')">
        <n-input
            v-model:value="steamcmd_pwd"
            :placeholder="$t('settings.steamPassword')"
            show-password-on="click"
            type="password"
        />
      </n-form-item>

      <n-form-item :label="$t('settings.parallelDownloads')">
        <n-input-number
            v-model:value="parallel"
            :max="10"
            :min="1"
            :placeholder="$t('settings.parallelDownloads')"
        />
      </n-form-item>
      <n-form-item :label="$t('settings.metadataRetrieveBatchsize')">
        <n-input-number
            v-model:value="config.metadataRetrieveBatchsize"
            :min="0"
            :placeholder="$t('settings.metadataRetrieveBatchsizePlaceholder')"
        />
      </n-form-item>
    </n-form>
  </n-card>
</template>

<script lang="ts" setup>
import { config, refresh_config } from "../../invokes.ts";
import { Level, SteamCmdConfig } from "../../proto/config.ts";
import { computed, onMounted } from "vue";
import { message, open } from "@tauri-apps/plugin-dialog";

const steamcmd_uname = computed({
	get: () => config.value.steamcmdConfig?.username ?? "",
	set: (newValue) => {
		if (config.value.steamcmdConfig) {
			config.value.steamcmdConfig.username = newValue;
		} else {
			config.value.steamcmdConfig = SteamCmdConfig.fromPartial({
				username: newValue,
			});
		}
	},
});

const steamcmd_pwd = computed({
	get: () => config.value.steamcmdConfig?.password ?? "",
	set: (newValue) => {
		if (config.value.steamcmdConfig) {
			config.value.steamcmdConfig.password = newValue;
		} else {
			config.value.steamcmdConfig = SteamCmdConfig.fromPartial({
				password: newValue,
			});
		}
	},
});

const parallel = computed({
	get: () => config.value.steamcmdConfig?.parallel ?? 0,
	set: (newValue) => {
		if (config.value.steamcmdConfig) {
			config.value.steamcmdConfig.parallel = newValue;
		} else {
			config.value.steamcmdConfig = SteamCmdConfig.fromPartial({
				parallel: newValue,
			});
		}
	},
});

const logLevelOptions = [
	{ label: "Trace", value: Level.Trace },
	{ label: "Debug", value: Level.Debug },
	{ label: "Info", value: Level.Info },
	{ label: "Warning", value: Level.Warn },
	{ label: "Error", value: Level.Error },
];

onMounted(refresh_config);
const loglevel = computed({
	get: () => config.value.loglevel,
	set: (newValue) => {
		if (newValue === Level.UNRECOGNIZED) {
			newValue = Level.Info;
		}
		config.value.loglevel = newValue;
	},
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
			config.value.gameHome = selected as string;
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
			config.value.steamcmdHome = selected as string;
		}
	} catch (error) {
		await showError("Failed to select SteamCMD path", error);
	}
};
</script>

