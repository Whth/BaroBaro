<template>
  <n-grid v-if="mod!=null && mod.previewImage!==undefined" cols="2" style="height: 61vh" y-gap="200px">

    <n-gi>
      <n-image :src="mod.previewImage" width="100%"/>

      <n-scrollbar style="max-height: 35vh">
        <bbob-bbcode>
          {{ mod.description }}
        </bbob-bbcode>
      </n-scrollbar>

    </n-gi>

    <n-gi>
      <n-descriptions :column="1">
        <n-descriptions-item :label="$t('modDetails.name')">
          <n-text>{{ mod.name }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.author')">
          <n-text>{{ mod.creator }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.lastModified')">
          <n-text>{{ formatTimestampToDate(mod.lastModified ?? 0) }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.size')">
          <n-text>{{ bytes(mod.size ?? 0) }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.subscribers')">
          <n-text>{{ mod.subscribers }}</n-text>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.likes')">
          <n-text>{{ mod.likes }}</n-text>
        </n-descriptions-item>
      </n-descriptions>

    </n-gi>
  </n-grid>
  <n-h4 v-else>
    {{ $t('modDetails.notSelected') }}
  </n-h4>
</template>


<script lang="ts" setup>
import type {BarotraumaMod} from '../../proto/mods.ts'
import bytes from "bytes";
import {Language, languageToJSON} from "../../proto/config.ts";


/**
 * Convert a timestamp in seconds to a date string in YYYY-MM-DD format
 * @param timestamp - Timestamp in seconds (e.g., 1751876035)
 * @returns Formatted date string, such as "2025-07-06"
 */
function formatTimestampToDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)


  return date.toLocaleDateString(languageToJSON(Language.ZH), {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  }).replace(/\//g, '-')
}

interface Props {
  mod: BarotraumaMod | null,
}

defineProps<Props>()
</script>