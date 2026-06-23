<template>
  <TitledPage>
    <template #title>
      <n-h1 v-text="$t('navigation.mods')"></n-h1>
    </template>

    <n-card>
      <ModManagementTabs/>
    </n-card>
  </TitledPage>
</template>

<script lang="ts" setup>
import TitledPage from "../../components/core/TitledPage.vue";
import ModManagementTabs from "../../components/mod-management/ModManagementTabs.vue";
</script>

