<template>
  <div class="mods-view">
    <n-space vertical :size="24">
      <div class="mods-header">
        <h1 class="page-title">{{ t('navigation.mods') }}</h1>
      </div>

      <n-card>
        <ModManagementTabs />
      </n-card>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { NSpace, NCard } from "naive-ui";
import ModManagementTabs from "../../components/mod-management/ModManagementTabs.vue";

const { t } = useI18n();
</script>

<style scoped>
.mods-view {
  height: 100%;
  overflow-y: auto;
}

.mods-header {
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
  height: calc(100vh - 200px);
  min-height: 400px;
}
</style>
