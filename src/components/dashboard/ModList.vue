<template>
  <n-grid :cols="1" :x-gap="8" :y-gap="12">
    <n-gi v-for="[index, mod] in enabled_mods.entries()" :key="mod.name">
      <ModCard :index="index" :mod="mod"></ModCard>
    </n-gi>
  </n-grid>
</template>


<script lang="ts" setup>
import {enabled_mods,} from "../../invokes.ts";
import ModCard from "./ModCard.vue";
</script>

