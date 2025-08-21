<template>
  <div class="dashboard-view staggered-animation">
    <div class="dashboard-header">
      <Title type="page">{{ t('navigation.dashboard') }}</Title>
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
import Title from "../../components/core/Title.vue";

const { t } = useI18n();
</script>

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
  max-height: calc(100vh - 2 * var(--spacing-l));
  overflow: hidden;
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
