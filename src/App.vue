<template>
  <n-config-provider :theme="currentTheme" autofocus>
    <div class="app-container">
      <Layout>
        <router-view/>
      </Layout>
    </div>
  </n-config-provider>
</template>

<script lang="ts" setup>
import {onMounted} from "vue";
import Layout from "./components/core/Layout.vue";
import {Theme} from "./proto/config.ts";
import {config, refresh_config} from "./invokes.ts";
import {currentTheme, theme_mapping} from "./composables/useTheme.ts";


onMounted(async () => {
  await refresh_config()
  currentTheme.value = theme_mapping[config.value.uiConfig?.theme || Theme.Light];
});
</script>

