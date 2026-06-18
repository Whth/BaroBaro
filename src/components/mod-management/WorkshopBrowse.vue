<template>
  <n-scrollbar style="max-height: 70vh">
    <div style="padding: 8px">
      <n-input
        v-model:value="searchQuery"
        :placeholder="$t('browse.searchPlaceholder')"
        clearable
        style="margin-bottom: 16px"
      >
        <template #prefix>
          <n-icon><search-outline /></n-icon>
        </template>
      </n-input>

      <n-spin :show="loading" style="width: 100%">
        <n-empty v-if="!loading && filteredMods.length === 0" :description="$t('browse.noResults')" />

        <div v-else class="browse-grid">
          <n-card
            v-for="mod in filteredMods"
            :key="mod.publishedFileId"
            :title="mod.title"
            hoverable
            size="small"
          >
            <template #cover>
              <img
                v-if="mod.previewUrl"
                :src="mod.previewUrl"
                :alt="mod.title"
                style="height: 120px; object-fit: cover; width: 100%"
              />
            </template>
            <div class="mod-meta">
              <n-tag v-if="mod.subscriptions > 0" size="small" type="info">
                {{ mod.subscriptions }} subs
              </n-tag>
              <n-tag v-if="mod.favorited > 0" size="small" type="warning">
                {{ mod.favorited }} favs
              </n-tag>
            </div>
            <template #action>
              <n-button size="small" type="primary" @click="handleDownload(mod.publishedFileId)">
                {{ $t('browse.download') }}
              </n-button>
            </template>
          </n-card>
        </div>
      </n-spin>
    </div>
  </n-scrollbar>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from "vue";
import { get_popular_mods, download_mods } from "../../invokes.ts";
import type { WorkshopItem } from "../../proto/workshop.ts";
import { SearchOutline } from "@vicons/ionicons5";
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";

const message = useMessage();
const { t } = useI18n();
const loading = ref(false);
const searchQuery = ref("");
const popularMods = ref<WorkshopItem[]>([]);

const filteredMods = computed(() => {
	const query = searchQuery.value.toLowerCase().trim();
	if (!query) return popularMods.value;
	return popularMods.value.filter(
		(m) => m.title.toLowerCase().includes(query) || m.description?.toLowerCase().includes(query),
	);
});

onMounted(async () => {
	loading.value = true;
	try {
		popularMods.value = await get_popular_mods();
	} catch {
		// Silent — advisory feature
	} finally {
		loading.value = false;
	}
});

async function handleDownload(id: number) {
	try {
		await download_mods([id]);
		message.success(t("browse.downloadStarted"));
	} catch (error) {
		message.error(String(error));
	}
}
</script>

<style scoped>
.browse-grid {
	display: grid;
	gap: 12px;
	grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
}

.mod-meta {
	display: flex;
	gap: 8px;
	margin-top: 8px;
}
</style>
