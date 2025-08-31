<template>
  <n-scrollbar style="height: 69vh">
    <n-grid :cols="1" :x-gap="8" :y-gap="12">
      <n-gi v-for="[index, mod] in installed_mod.entries()" :key="mod.name">
        <ModCard :index="index" :mod="mod" @mod-selected="viewMod"></ModCard>
      </n-gi>
    </n-grid>
    <n-empty v-if="installed_mod.length === 0" :description="$t('modList.empty')" style="margin-top: 20px">
      <template #icon>
        <n-icon>
          <archive-outline/>
        </n-icon>
      </template>
    </n-empty>
  </n-scrollbar>
</template>


<script lang="ts" setup>
import { installed_mod } from "../../invokes.ts";
import ModCard from "./ModCard.vue";
import type { BarotraumaMod } from "../../proto/mods.ts";
import { ArchiveOutline } from "@vicons/ionicons5";

const emit = defineEmits(["viewingMod"]);

function viewMod(mod: BarotraumaMod) {
	emit("viewingMod", mod);
}
</script>

