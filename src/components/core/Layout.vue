<template>
  <n-layout class="layout" has-sider style="height: 100vh">
    <Navigation/>
    <n-layout>
      <n-alert
        v-if="networkStatus && !networkStatus.steamApi"
        type="warning"
        :title="$t('dashboard.offline')"
        :bordered="false"
        closable
        style="margin: 0"
      >
        {{ $t('dashboard.offlineDescription') }}
      </n-alert>
      <n-scrollbar>
        <n-layout-content style="padding-left: 3%;padding-right: 3%;height: 100vh">
          <slot/>
        </n-layout-content>
      </n-scrollbar>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import Navigation from "./Navigation.vue";
import { check_network_status } from "../../invokes.ts";
import type { NetworkStatus } from "../../invokes.ts";
import { onMounted, ref } from "vue";

const networkStatus = ref<NetworkStatus | null>(null);

onMounted(async () => {
	try {
		networkStatus.value = await check_network_status();
	} catch {
		networkStatus.value = { steamApi: false, steamcmdAvailable: false };
	}
});
</script>

