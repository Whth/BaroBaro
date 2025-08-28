<template>
  <n-card :title="mod.name">
    <n-text></n-text>
  </n-card>


</template>
<script lang="ts" setup>
import { BarotraumaMod } from "../../proto/mods.ts";

defineProps<{
	mod: BarotraumaMod;
}>();
</script>


<style scoped>

</style>