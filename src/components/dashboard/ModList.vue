<template>
  <n-card :bordered="false" class="mod-list-card">
    <n-h2>Installed Mods</n-h2>

    <n-list-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
      <ModCard :mod="mod"></ModCard>
    </n-list-item>
  </n-card>
</template>

<script lang="ts" setup>
import {installed_mod,} from "../../invokes.ts";
import ModCard from "./ModCard.vue";
</script>

