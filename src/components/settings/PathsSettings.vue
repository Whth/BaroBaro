<template>
  <div class="paths-settings">

    <n-card segmented title="Game & SteamCMD Paths">
      <n-form :model="settings" label-placement="left" label-width="200">
        <n-form-item label="Game Installation Path">
          <n-input-group>
            <n-input v-model:value="settings.gamePath" placeholder="Select game installation path" readonly/>
            <n-button @click="browseGamePath">Browse</n-button>
          </n-input-group>
        </n-form-item>

        <n-form-item label="SteamCMD Path">
          <n-input-group>
            <n-input v-model:value="settings.steamcmdPath" placeholder="Select SteamCMD path" readonly/>
            <n-button @click="browseSteamCmdPath">Browse</n-button>
          </n-input-group>
        </n-form-item>
      </n-form>
    </n-card>

    <n-card segmented title="SteamCMD Configuration">
      <n-form :model="settings" label-placement="left" label-width="200">
        <n-form-item label="Steam Username">
          <n-input v-model:value="settings.steamcmdUsername" placeholder="Enter Steam username"/>
        </n-form-item>

        <n-form-item label="Steam Password">
          <n-input
              v-model:value="settings.steamcmdPassword"
              placeholder="Enter Steam password"
              show-password-on="click"
              type="password"
          />
        </n-form-item>

        <n-form-item label="Parallel Downloads">
          <n-input-number
              v-model:value="settings.steamcmdParallel"
              :max="10"
              :min="1"
              placeholder="Number of parallel downloads"
          />
        </n-form-item>
      </n-form>
    </n-card>

    <n-space justify="end" style="margin-top: 20px">
      <n-button type="primary" @click="saveSettings">Save Paths</n-button>
    </n-space>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, ref} from "vue";
import {useModManager} from "../../composables/useModManager";
import {message, open} from "@tauri-apps/plugin-dialog";

const {config, updateGameHome, updateSteamCmdHome} = useModManager();

const settings = ref({
  gamePath: "",
  steamcmdPath: "",
  steamcmdUsername: "",
  steamcmdPassword: "",
  steamcmdParallel: 1,
});

const showError = async (title: string, error: any) => {
  console.error(title, error);
  await message(`Error: ${error.message || error}`, {title, kind: "error"});
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

const saveSettings = async () => {
  try {
    if (settings.value.gamePath) await updateGameHome(settings.value.gamePath);
    if (settings.value.steamcmdPath) await updateSteamCmdHome(settings.value.steamcmdPath);

    // TODO: Save SteamCMD config if needed
    console.log("Settings to save:", settings.value);

    await message("Paths settings saved successfully!", {title: "Success", kind: "info"});
  } catch (error) {
    await showError("Failed to save paths settings", error);
  }
};

onMounted(() => {
  if (config.value) {
    settings.value.gamePath = config.value.gameHome;
    settings.value.steamcmdPath = config.value.steamcmdHome;
    if (config.value.steamcmdConfig) {
      settings.value.steamcmdUsername = config.value.steamcmdConfig.username;
      settings.value.steamcmdPassword = config.value.steamcmdConfig.password;
      settings.value.steamcmdParallel = config.value.steamcmdConfig.parallel;
    }
  }
});
</script>

<style scoped>
.paths-settings {
  padding: 20px;
}
</style>