<template>
  <n-scrollbar style="max-height: 60vh">
    <n-list :bordered="false" style="background-color: transparent">
      <n-list-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
        <ModItem :mod="mod"/>
      </n-list-item>
    </n-list>
  </n-scrollbar>
</template>

<script lang="ts" setup>
import { installed_mod } from "../../invokes.ts";
import ModItem from "./ModItem.vue";
</script>



