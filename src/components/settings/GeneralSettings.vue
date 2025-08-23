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

      <div class="form-actions">
        <n-button type="primary" @click="save_config" v-text="$t('settings.save')">
        </n-button>
      </div>
    </n-form>
  </n-card>
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
