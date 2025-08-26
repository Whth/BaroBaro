<template>
  <n-scrollbar style="max-height: 61vh">
    <n-grid :cols="1" :x-gap="8" :y-gap="12">
      <n-gi v-for="[index, mod] in enabled_mods.entries()" :key="mod.name">
        <ModCard :index="index" :mod="mod" @mod-selected="viewMod"></ModCard>
      </n-gi>
    </n-grid>
  </n-scrollbar>
</template>


<script lang="ts" setup>
import {enabled_mods,} from "../../invokes.ts";
import ModCard from "./ModCard.vue";
import {BarotraumaMod} from "../../proto/mods.ts";

const emit = defineEmits(["viewingMod"]);

function viewMod(mod: BarotraumaMod) {
  emit("viewingMod", mod);
}

</script>

