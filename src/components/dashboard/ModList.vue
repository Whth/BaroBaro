<template>
  <n-grid :cols="1" :x-gap="8" :y-gap="12">
    <n-gi v-for="mod in installed_mod" :key="mod.name">
      <ModCard :mod="mod"></ModCard>
    </n-gi>
  </n-grid>
</template>


<script lang="ts" setup>
import {installed_mod,} from "../../invokes.ts";
import ModCard from "./ModCard.vue";
</script>

