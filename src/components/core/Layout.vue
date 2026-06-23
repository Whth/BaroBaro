<template>
  <n-layout class="layout" has-sider style="height: 100vh">
    <Navigation/>
    <n-layout>
      <Transition name="fade">
        <n-alert
            v-if="showOfflineAlert && networkStatus && !networkStatus.steamApi"
            :bordered="false"
            :title="$t('dashboard.offline')"
            closable
            style="margin: 0"
            type="warning"
            @close="showOfflineAlert = false"
        >
          {{ $t('dashboard.offlineDescription') }}
        </n-alert>
      </Transition>
      <n-scrollbar>
        <n-layout-content style="padding-left: 3%;padding-right: 3%;">
          <slot/>
        </n-layout-content>
      </n-scrollbar>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from "vue";
import type { NetworkStatus } from "../../invokes.ts";
import { check_network_status } from "../../invokes.ts";
import Navigation from "./Navigation.vue";

const networkStatus = ref<NetworkStatus | null>(null);
const showOfflineAlert = ref(false);
let dismissTimer: ReturnType<typeof setTimeout> | null = null;

onMounted(async () => {
	try {
		networkStatus.value = await check_network_status();
	} catch {
		networkStatus.value = { steamApi: false, steamcmdAvailable: false };
	}

	if (networkStatus.value && !networkStatus.value.steamApi) {
		showOfflineAlert.value = true;
		dismissTimer = setTimeout(() => {
			showOfflineAlert.value = false;
		}, 8000);
	}
});

onUnmounted(() => {
	if (dismissTimer) clearTimeout(dismissTimer);
});
</script>

<style scoped>
.fade-enter-active {
	transition: opacity 0.5s ease;
}
.fade-leave-active {
	transition: opacity 1.5s ease;
}
.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}
</style>
