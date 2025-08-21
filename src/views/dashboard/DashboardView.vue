<template>
  <div class="dashboard-view">
    <div class="dashboard-header">
      <h1>{{ t('navigation.dashboard') }}</h1>
    </div>
    <SearchAndFilter />
    <ModList />
    <ModDetails />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import SearchAndFilter from "../../components/dashboard/SearchAndFilter.vue";
import ModList from "../../components/dashboard/ModList.vue";
import ModDetails from "../../components/dashboard/ModDetails.vue";

const { t } = useI18n();
</script>

<style scoped>
.dashboard-view {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-l);
  background: transparent;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dashboard-header h1 {
  margin: 0;
}
</style>
