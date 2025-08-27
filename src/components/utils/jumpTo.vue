<template>
  <slot/>
  <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
    <n-icon size="1.2em"
            @click="openUrl(url)">
      <open-outline/>
    </n-icon>
  </n-button>
</template>

<script lang="ts" setup>
import { OpenOutline } from "@vicons/ionicons5";
import { openUrl } from "@tauri-apps/plugin-opener";

interface Props {
	url: string;
}

defineProps<Props>();
</script>
