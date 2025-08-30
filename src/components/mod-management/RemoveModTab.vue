<template>
  <n-card :bordered="false" class="remove-mod-card">
    <n-h2>Remove Mods</n-h2>
    <n-list :bordered="false" style="background-color: transparent">

      <n-list-item v-for="mod in installed_mod" :key="mod.steamWorkshopId">
        <ModItem :mod="mod"/>
      </n-list-item>
    </n-list>
  </n-card>
</template>

<script lang="ts" setup>
import { installed_mod } from "../../invokes.ts";
import ModItem from "./ModItem.vue";
</script>



