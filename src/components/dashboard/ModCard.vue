<template>
  <n-card hoverable style="cursor: pointer;" @click="onClick()">
    <n-thing>
      <template #header>
        <div style="display: flex; align-items: center; gap: 12px; flex-wrap: wrap;">

          <n-text strong style="font-size: 16px; ">{{ `${index}. ` }}</n-text>
          <n-text strong style="font-size: 16px;">{{ mod.name }}</n-text>
          <n-tag v-if="mod.corePackage" size="small" type="info" v-text="$t('modCard.corePackage')"></n-tag>
          <n-tag v-if="mod.homeDir" size="small" type="success" v-text="$t('modCard.localMod')"></n-tag>
        </div>
      </template>
      <template #description>
        <n-flex :size="6" style="margin-top: 8px;" vertical>
          <n-descriptions :column="4" size="small">
            <!-- Mod Version -->
            <n-descriptions-item :label="$t('modCard.version')">
              <inlineCode :displayText="mod.modVersion"></inlineCode>
            </n-descriptions-item>

            <!-- Game Version -->
            <n-descriptions-item :label="$t('modCard.gameVersion')">
              <inlineCode :displayText="mod.gameVersion"></inlineCode>
            </n-descriptions-item>

            <!-- Steam Workshop ID -->
            <n-descriptions-item :label="$t('modCard.steamWorkshopId')">
              <inline-code :displayText="mod.steamWorkshopId.toString()"></inline-code>
              <JumpTo :url="`https://steamcommunity.com/sharedfiles/filedetails/?id=${mod.steamWorkshopId}`"></JumpTo>
            </n-descriptions-item>
            <!-- Expected Hash -->
            <n-descriptions-item :label="$t('modCard.expectedHash')">
              <inline-code :displayText="`${mod.expectedHash.slice(0,7)}...`" :src="mod.expectedHash"></inline-code>
            </n-descriptions-item>
            <!-- Home Directory -->
            <n-descriptions-item v-if="mod.homeDir" :label="$t('modCard.homeDir')">
              <inline-code :displayText="mod.homeDir"></inline-code>
              <reveal :path="mod.homeDir"></reveal>
            </n-descriptions-item>
          </n-descriptions>
        </n-flex>
      </template>
    </n-thing>
  </n-card>
</template>

<script lang="ts" setup>
import type { BarotraumaMod } from "../../proto/mods.ts";
import InlineCode from "../utils/inlineCode.vue";
import JumpTo from "../utils/jumpTo.vue";
import Reveal from "../utils/Reveal.vue";

interface Props {
	mod: BarotraumaMod;
	index: number;
}

const props = defineProps<Props>();

type Emits = (e: "modSelected", data: BarotraumaMod) => void;

const emit = defineEmits<Emits>();

const onClick = () => {
	console.log(`Selected mod: ${props.mod.name}`);
	emit("modSelected", props.mod);
};
</script>