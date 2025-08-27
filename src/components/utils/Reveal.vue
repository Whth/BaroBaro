<template>
  <slot/>
  <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
    <n-icon size="1.2em" @click="revealItemInDir(path)">
      <folder-open-outline/>
    </n-icon>
  </n-button>
</template>

<script lang="ts" setup>
import { FolderOpenOutline } from "@vicons/ionicons5";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

interface Props {
	path: string;
}

defineProps<Props>();
</script>
