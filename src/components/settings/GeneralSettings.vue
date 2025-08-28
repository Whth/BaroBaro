<template>
  <n-grid :cols="1" :x-gap="16" :y-gap="16">


    <n-gi>
      <n-card class="animate-fade-in">
        <template #header>
          <n-h3>{{ $t('settings.gameAndSteam') }}</n-h3>
        </template>

        <n-form :label-width="180">
          <n-form-item :label="$t('settings.gameHome')">
            <n-input-group>
              <n-input
                  v-model:value="config.gameHome"
                  :placeholder="$t('settings.gameHomePlaceholder')"
                  readonly
              />
              <n-button @click="browseGamePath">{{ $t('settings.browse') }}</n-button>
            </n-input-group>
          </n-form-item>

          <n-form-item :label="$t('settings.steamCmdHome')">
            <n-input-group>
              <n-input
                  v-model:value="config.steamcmdHome"
                  :placeholder="$t('settings.steamCmdHomePlaceholder')"
                  readonly
              />
              <n-button @click="browseSteamCmdPath">{{ $t('settings.browse') }}</n-button>
            </n-input-group>
          </n-form-item>

          <n-form-item :label="$t('settings.steamUsername')">
            <n-input
                v-model:value="steamcmd_uname"
                :placeholder="$t('settings.steamUsernamePlaceholder')"
            />
          </n-form-item>

          <n-form-item :label="$t('settings.steamPassword')">
            <n-input
                v-model:value="steamcmd_pwd"
                :placeholder="$t('settings.steamPasswordPlaceholder')"
                show-password-on="click"
                type="password"
            />
          </n-form-item>

          <n-form-item :label="$t('settings.parallelDownloads')">
            <n-input-number
                v-model:value="parallel"
                :max="10"
                :min="1"
                :placeholder="$t('settings.parallelDownloadsPlaceholder')"
            />
          </n-form-item>


          <n-form-item :label="$t('settings.installStrategy')">
            <n-select
                v-model:value="installStrategy"
                :options="installStrategyOptions"
                style="width: 200px"
            />
          </n-form-item>
        </n-form>
      </n-card>
    </n-gi>

    <n-gi>
      <n-card class="form-section animate-fade-in">
        <template #header>
          <n-h3>{{ $t('settings.loggingAndAdvanced') }}</n-h3>
        </template>

        <n-form :label-width="220">
          <n-form-item :label="$t('settings.logLevel')">
            <n-select
                v-model:value="loglevel"
                :options="logLevelOptions"
                style="width: 200px"
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
    </n-gi>
  </n-grid>
</template>

<script lang="ts" setup>
import { computed, onMounted } from "vue";
import { config, refresh_config } from "../../invokes.ts";
import { message, open } from "@tauri-apps/plugin-dialog";
import {
	InstallStrategy,
	installStrategyToJSON,
	Level,
	levelToJSON,
	SteamCmdConfig,
} from "../../proto/config.ts";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const loglevel = computed({
	get: () => config.value.loglevel,
	set: (newValue) => {
		if (newValue === Level.UNRECOGNIZED) {
			newValue = Level.Info;
		}
		config.value.loglevel = newValue;
	},
});

const installStrategy = computed({
	get: () => config.value.installStrategy,
	set: (newValue) => {
		if (newValue === InstallStrategy.UNRECOGNIZED) {
			newValue = InstallStrategy.Copy;
		}
		config.value.installStrategy = newValue;
	},
});

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
	get: () => config.value.steamcmdConfig?.parallel ?? 1,
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
	{ label: levelToJSON(Level.Trace), value: Level.Trace },
	{ label: levelToJSON(Level.Debug), value: Level.Debug },
	{ label: levelToJSON(Level.Info), value: Level.Info },
	{ label: levelToJSON(Level.Warn), value: Level.Warn },
	{ label: levelToJSON(Level.Error), value: Level.Error },
];

const installStrategyOptions = [
	{
		label: installStrategyToJSON(InstallStrategy.Copy),
		value: InstallStrategy.Copy,
	},
	{
		label: installStrategyToJSON(InstallStrategy.Link),
		value: InstallStrategy.Link,
	},
];

const showError = async (title: string, error: any) => {
	console.error(title, error);
	await message(`Error: ${error.message || error}`, { title, kind: "error" });
};

const browseGamePath = async () => {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
			title: t("settings.selectGamePathTitle"),
		});

		if (selected) {
			config.value.gameHome = selected as string;
		}
	} catch (error) {
		await showError(t("settings.gamePathError"), error);
	}
};

const browseSteamCmdPath = async () => {
	try {
		const selected = await open({
			directory: true,
			multiple: false,
			title: t("settings.selectSteamCmdPathTitle"),
		});

		if (selected) {
			config.value.steamcmdHome = selected as string;
		}
	} catch (error) {
		await showError(t("settings.steamCmdPathError"), error);
	}
};

onMounted(refresh_config);
</script>
