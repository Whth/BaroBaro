<template>
  <div class="general-settings">
    <n-h2>General Settings</n-h2>

    <n-card class="settings-card">
      <n-form class="settings-form">
        <n-form-item label="Log Level" path="logLevel">
          <n-select
              v-model:value="loglevel"
              :options="logLevelOptions"
              style="width: 200px"
          />
        </n-form-item>

        <div class="form-actions">
          <n-button type="primary" @click="save_config">
            Save Settings
          </n-button>
        </div>
      </n-form>
    </n-card>
  </div>
</template>

<script lang="ts" setup>
import {config, refresh_config, save_config} from "../../invokes.ts";
import {Level} from "../../proto/config.ts";
import {computed, onMounted} from "vue";

const loglevel = computed({
  get: () => config.value.loglevel,
  set: (newValue) => {
    if (newValue === Level.UNRECOGNIZED) {
      newValue = Level.Info
    }
    config.value.loglevel = newValue
  }
})


const logLevelOptions = [
  {label: 'Trace', value: Level.Trace},
  {label: 'Debug', value: Level.Debug},
  {label: 'Info', value: Level.Info},
  {label: 'Warning', value: Level.Warn},
  {label: 'Error', value: Level.Error}
];

onMounted(refresh_config);
</script>

<style scoped>
.general-settings {
  padding: 20px;
}

.settings-card {
  max-width: 600px;
}

.settings-form {
  margin-top: 20px;
}

.form-actions {
  margin-top: 30px;
}
</style>