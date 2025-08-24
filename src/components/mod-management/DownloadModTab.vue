<template>
  <n-card :bordered="false" class="download-mod-card">
    <n-h2>Download Mods from Steam Workshop</n-h2>

    <!-- 搜索和过滤区域 -->
    <n-space vertical>
      <n-input-group>
        <n-input
            v-model:value="searchQuery"
            clearable
            placeholder="Search for mods on Steam Workshop..."
        >
          <template #prefix>
            <n-icon>
              <SearchOutline/>
            </n-icon>
          </template>
        </n-input>
        <n-button type="primary">Search</n-button>
      </n-input-group>

      <n-space>
        <n-select
            v-model:value="categoryFilter"
            :options="categoryOptions"
            placeholder="Category"
            style="width: 150px"
        />
        <n-select
            v-model:value="sortBy"
            :options="sortOptions"
            placeholder="Sort by"
            style="width: 150px"
        />
      </n-space>
    </n-space>

    <!-- 加载状态 -->
    <n-spin :show="loading">
      <!-- 无数据状态 -->
      <n-result
          v-if="mods.length === 0 && !loading"
          description="Try adjusting your search criteria"
          status="info"
          title="No mods found"
      />

      <!-- Mod网格 -->
      <div v-else class="mod-grid">
        <n-card
            v-for="mod in mods"
            :key="mod.id"
            class="mod-card"
            hoverable
            size="small"
        >
          <template #header>
            <n-ellipsis style="max-width: 200px">
              {{ mod.name }}
            </n-ellipsis>
          </template>

          <template #header-extra>
            <n-tag type="default">{{ mod.version }}</n-tag>
          </template>

          <n-ellipsis :line-clamp="2" style="margin-bottom: 12px">
            {{ mod.description }}
          </n-ellipsis>

          <n-space size="small" vertical>
            <n-text depth="3">by {{ mod.author }}</n-text>
            <n-space justify="space-between">
              <n-text depth="3">📥 {{ formatNumber(mod.downloadCount) }}</n-text>
              <n-rate :allow-half="true" :value="mod.rating" readonly size="small"/>
            </n-space>
          </n-space>

          <template #action>
            <n-button
                :loading="mod.downloading"
                block
                secondary
                size="small"
                type="primary"
                @click="downloadMod(mod.id)"
            >
              {{ mod.downloading ? 'Downloading...' : 'Download' }}
            </n-button>
          </template>
        </n-card>
      </div>
    </n-spin>
  </n-card>
</template>

<script lang="ts" setup>
import {ref} from "vue";
import {SearchOutline} from "@vicons/ionicons5";
import type {SelectOption} from "naive-ui";

// 定义接口
interface Mod {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  downloadCount: number;
  category: string;
  rating: number;
  downloading?: boolean;
}

// 响应式数据
const searchQuery = ref("");
const categoryFilter = ref("all");
const sortBy = ref("popular");
const loading = ref(false);

// 下拉选项
const categoryOptions: SelectOption[] = [
  {label: "All Categories", value: "all"},
  {label: "Graphics", value: "graphics"},
  {label: "Audio", value: "audio"},
  {label: "Gameplay", value: "gameplay"},
  {label: "Utility", value: "utility"},
];

const sortOptions: SelectOption[] = [
  {label: "Most Popular", value: "popular"},
  {label: "Newest", value: "newest"},
  {label: "Highest Rated", value: "rating"},
];

// 模拟数据
const mods = ref<Mod[]>([
  {
    id: "1",
    name: "Enhanced Graphics Pack",
    version: "2.1.0",
    author: "VisualMaster",
    description: "High resolution textures and improved lighting effects",
    downloadCount: 125000,
    category: "graphics",
    rating: 4.8,
  },
  {
    id: "2",
    name: "Immersive Soundscapes",
    version: "1.5.3",
    author: "AudioWizard",
    description: "Realistic ambient sounds and improved audio effects",
    downloadCount: 87500,
    category: "audio",
    rating: 4.6,
  },
  {
    id: "3",
    name: "Advanced Crafting System",
    version: "3.0.1",
    author: "CraftExpert",
    description: "Revamped crafting mechanics with new recipes and items",
    downloadCount: 210000,
    category: "gameplay",
    rating: 4.9,
  },
]);

// 下载mod
const downloadMod = async (modId: string) => {
  const mod = mods.value.find((m) => m.id === modId);
  if (!mod) return;

  mod.downloading = true;
  try {
    // 调用下载函数
    await downloadMods([parseInt(modId, 10)]);
    window.$message?.success(`Downloaded ${mod.name} successfully!`);
  } catch (error) {
    window.$message?.error(`Failed to download ${mod.name}`);
  } finally {
    mod.downloading = false;
  }
};

// 格式化数字显示
const formatNumber = (num: number) => {
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
  return num.toString();
};

// 模拟加载
loading.value = true;
setTimeout(() => {
  loading.value = false;
}, 1000);
</script>

<style scoped>
.download-mod-card {
  height: 100%;
}

.mod-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
  margin-top: 20px;
}

.mod-card {
  height: fit-content;
}
</style>