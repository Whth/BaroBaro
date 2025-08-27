<template>
  <n-grid v-if="mod!=null && mod.previewImage!==undefined" cols="7" style="height: 61vh" x-gap="20" y-gap="2vh">
    <n-gi span="4">

      <n-image :lazy="true" :src="mod.previewImage" width="100%">
        <template #error>
          <n-icon :size="100" color="lightGrey">
            <ImageOutline/>
          </n-icon>
        </template>
      </n-image>
    </n-gi>

    <n-gi span="3">
      <n-descriptions :column="2">
        <n-descriptions-item :label="$t('modDetails.name')" span="2">
          <n-text code>{{ mod.name }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.author')" span="2">
          <n-text code>{{ mod.creator }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.lastModified')">
          <n-text code>{{ formatTimestampToDate(mod.lastModified ?? 0) }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.subscribers')">
          <n-text code>{{ mod.subscribers }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.size')">
          <n-text code>{{ bytes(mod.size ?? 0) }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.likes')">
          <n-text code>{{ mod.likes }}</n-text>
        </n-descriptions-item>
      </n-descriptions>

    </n-gi>

  </n-grid>
  <n-h4 v-else>
    {{ $t('modDetails.notSelected') }}
  </n-h4>
</template>


<script lang="ts" setup>
import type { BarotraumaMod } from "../../proto/mods.ts";
import { ImageOutline } from "@vicons/ionicons5";
import bytes from "bytes";
import { Language, languageToJSON } from "../../proto/config.ts";
import { onErrorCaptured } from "vue";

onErrorCaptured((err, info) => {
	console.warn("[BBob Error]", err, info);
	return false; // 阻止错误继续冒泡
});

/**
 * Convert a timestamp in seconds to a date string in YYYY-MM-DD format
 * @param timestamp - Timestamp in seconds (e.g., 1751876035)
 * @returns Formatted date string, such as "2025-07-06"
 */
function formatTimestampToDate(timestamp: number): string {
	const date = new Date(timestamp * 1000);

	return date
		.toLocaleDateString(languageToJSON(Language.ZH), {
			year: "numeric",
			month: "2-digit",
			day: "2-digit",
		})
		.replace(/\//g, "-");
}

interface Props {
	mod: BarotraumaMod | null;
}

const props = defineProps<Props>();
</script>