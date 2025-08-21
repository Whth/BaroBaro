<template>
  <div class="dashboard-view">
    <n-space vertical :size="24">
      <div class="dashboard-header">
        <h1 class="page-title">{{ t('navigation.dashboard') }}</h1>
      </div>

      <n-card>
        <SearchAndFilter />
      </n-card>

      <n-grid x-gap="24" y-gap="24" cols="12">
        <n-grid-item span="8">
          <n-card title="Mods List">
            <ModList />
          </n-card>
        </n-grid-item>

        <n-grid-item span="4">
          <n-card title="Mod Details">
            <ModDetails />
          </n-card>
        </n-grid-item>
      </n-grid>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { NCard, NSpace, NGrid, NGridItem } from "naive-ui";
import SearchAndFilter from "../../components/dashboard/SearchAndFilter.vue";
import ModList from "../../components/dashboard/ModList.vue";
import ModDetails from "../../components/dashboard/ModDetails.vue";

const { t } = useI18n();
</script>

<style scoped>
.dashboard-view {
  height: 100%;
  overflow-y: auto;
}

.dashboard-header {
  margin-bottom: 24px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  color: var(--n-color-text-primary);
  margin: 0;
}

:deep(.n-card) {
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

:deep(.n-card-header) {
  font-weight: 600;
  font-size: 16px;
}
</style>
