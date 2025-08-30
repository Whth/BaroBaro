<template>

  <n-grid v-if="mod!=null && mod.previewImage!==undefined" cols="7" style="height: 65vh" x-gap="20" y-gap="2vh">

    <n-gi span="4">
      <n-image v-show="imageRendered" :lazy="false" :src="mod.previewImage" width="100%"
               @load="imageRendered=true">
        <template #error>
          <n-icon color="lightGrey" size="10vw" style="align-content: center">
            <ImageOutline/>
          </n-icon>
        </template>
      </n-image>
      <n-skeleton v-show="!imageRendered" :sharp="false" :size="'large'" style="height: 80% ;width: 100%"></n-skeleton>
    </n-gi>

    <n-gi v-if="!!mod.creator" span="3">
      <n-descriptions :column="2">
        <n-descriptions-item :label="$t('modDetails.name')" span="2">
          <inline-code :displayText="mod.name"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.author')" span="2">
          <inline-code :displayText="(mod.creator ?? 0).toString()"/>
          <JumpTo :url="`https://steamcommunity.com/profiles/${mod.creator}`"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.lastModified')">
          <inline-code :displayText="formatTimestampToDate(mod.lastModified ?? 0)"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.subscribers')">
          <inline-code :displayText="abbreviate(mod.subscribers ?? 0).toString()"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.size')">
          <inline-code :displayText="bytes(mod.size ?? 0) ?? 'null'"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modDetails.likes')">
          <inline-code :displayText="abbreviate(mod.likes ?? 0).toString()"/>
        </n-descriptions-item>
      </n-descriptions>

    </n-gi>
    <!-- Description skeleton -->
    <n-gi v-else span="3">
      <n-skeleton :height="24" :repeat="6" text/>
    </n-gi>


    <n-gi v-if="mod.tags" span="7">
      <n-flex>
        <n-tag
            v-for="tag in mod.tags"
            :key="tag"
            :color="getTagColorConfig(tag)"
            :style="{
      cursor: 'pointer',
      transition: 'all 0.2s ease',
      opacity: '0.85'
    }"
            class="mod-tag"
            round
            size="medium"
            @click="openUrl(`https://steamcommunity.com/workshop/browse/?appid=602960&requiredtags[]=${encodeURIComponent(tag)}`)"
        >
          {{ tag }}
        </n-tag>
      </n-flex>
    </n-gi>
    <!-- Tags skeleton -->
    <n-gi v-else span="7">
      <n-space>
        <n-skeleton v-for="i in 4" :key="i" height="1.7em" round width="60px"/>
      </n-space>
    </n-gi>
  </n-grid>
  <n-h4 v-else class="animate-bounce">
    {{ $t('modDetails.notSelected') }}
  </n-h4>
</template>


<script lang="ts" setup>
import abbreviate from "number-abbreviate";
import type { BarotraumaMod } from "../../proto/mods.ts";
import { ImageOutline } from "@vicons/ionicons5";
import bytes from "bytes";
import { Language, languageToJSON } from "../../proto/config.ts";
import getTagColorConfig from "../../composables/coloredTag.ts";
import InlineCode from "../utils/inlineCode.vue";
import JumpTo from "../utils/jumpTo.vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ref, watch } from "vue";

const imageRendered = ref(false);

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
watch(
	() => props.mod,
	() => {
		imageRendered.value = false;
	},
	{ immediate: false },
);
</script>