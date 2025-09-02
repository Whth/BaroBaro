<template>


  <n-grid v-if="item.retrieved" cols="10" x-gap="10">

    <n-gi span="3">
      <n-image :src="item.retrieved.previewUrl" width="100%"/>


    </n-gi>
    <n-gi span="7">
      <n-space vertical>

        <div style="display: flex; align-items: center; justify-content: space-between;">
          <n-h5 style="margin: 0; flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
            {{ item.retrieved.title }}
          </n-h5>
          <n-tag
              class="mod-tag"
              style="margin-left: 12px; flex-shrink: 0;"
              type="default"
              @click="openInWorkshop(item.retrieved.publishedFileId)"
          >
            {{ item.retrieved.publishedFileId }}
          </n-tag>
        </div>

        <n-flex>
          <n-tag :type="getStatusType(item.status)">
            {{ $t('workshop.status.' + item.status) }}
          </n-tag>
          <n-tag type="success">
            {{ $t('workshop.verified') }}
          </n-tag>
        </n-flex>
        <n-descriptions column="4">
          <n-descriptions-item :label="$t('workshop.size')">
            <inline-code :display-text="bytes(item.retrieved.fileSize) ?? ''"/>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('workshop.lastUpdated')">
            <inline-code :display-text="formatTimestampToDate(item.retrieved.timeUpdated)"/>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('workshop.subscriptions')">
            <inline-code :display-text="abbreviate(item.retrieved.subscriptions)"/>
          </n-descriptions-item>
          <n-descriptions-item :label="$t('workshop.favorited')">
            <inline-code :display-text="abbreviate(item.retrieved.favorited)"/>
          </n-descriptions-item>
        </n-descriptions>

      </n-space>
    </n-gi>
  </n-grid>
  <n-thing v-else column="4" label-align="left">
    <template #header>
      <n-h5>
        {{ item.url ?? item.id ?? 'Unknown' }}
      </n-h5>
    </template>

    <template #description>
      <n-flex>
        <n-tag :type="getStatusType(item.status)">
          {{ $t('workshop.status.' + item.status) }}
        </n-tag>
        <n-tag type="error">
          {{ $t('workshop.verifyFailed') }}
        </n-tag>
      </n-flex>
    </template>
  </n-thing>

</template>


<script lang="ts" setup>
import {
	formatTimestampToDate,
	openInWorkshop,
} from "../../composables/utils.ts";
import bytes from "bytes";
import InlineCode from "../utils/inlineCode.vue";
import abbreviate from "number-abbreviate";
import { getStatusType, ModItem } from "../../composables/workshop.ts";

defineProps<{
	item: ModItem;
}>();
</script>
