<template>
  <n-scrollbar style="max-height: 70vh">
    <n-grid :cols="1" :x-gap="8" :y-gap="12">
      <n-gi v-for="[index, mod] in installed_mod.entries()" :key="mod.name">
        <ModCard :index="index" :mod="mod" @mod-selected="viewMod"></ModCard>
      </n-gi>
    </n-grid>
  </n-scrollbar>
</template>


<script lang="ts" setup>
import { installed_mod } from "../../invokes.ts";
import ModCard from "./ModCard.vue";
import type { BarotraumaMod } from "../../proto/mods.ts";

const emit = defineEmits(["viewingMod"]);

function viewMod(mod: BarotraumaMod) {
	emit("viewingMod", mod);
}
</script>

