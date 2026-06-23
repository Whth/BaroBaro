<template>
  <n-card 
    class="mod-card" 
    :class="[`stagger-${Math.min(index, 6)}`]"
    hoverable 
    @click="onClick()"
  >
    <n-thing>
      <template #header>
        <div class="mod-card-header">
          <div class="mod-status-indicator" :class="is_enabled(mod) ? 'enabled' : 'disabled'" />
          <n-text strong class="mod-card-title">{{ mod.name }}</n-text>
          <n-tag 
            v-if="mod.corePackage" 
            size="small" 
            type="info" 
            round
            class="mod-badge"
          >
            {{ $t('modCard.corePackage') }}
          </n-tag>
        </div>
      </template>
      <template #description>
        <n-flex :size="8" align="center">
          <n-tag 
            v-if="is_enabled(mod)" 
            size="small" 
            type="success" 
            round
            :bordered="false"
          >
            <n-icon size="14" style="margin-right: 4px;"><CheckmarkCircleOutline /></n-icon>
            {{ $t('modCard.enabled') }}
          </n-tag>
          <n-tag 
            v-else 
            size="small" 
            type="warning" 
            round
          >
            <n-icon size="14" style="margin-right: 4px;"><EllipseOutline /></n-icon>
            {{ $t('modCard.disabled') }}
          </n-tag>
          <n-tag 
            v-if="mod.homeDir" 
            size="small" 
            type="default" 
            round
          >
            <n-icon size="14" style="margin-right: 4px;"><FolderOutline /></n-icon>
            {{ $t('modCard.localMod') }}
          </n-tag>
        </n-flex>
      </template>
      <template #default>
        <n-flex :size="8" style="margin-top: 12px;" vertical>
          <n-descriptions :column="4" size="small" label-placement="left">
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
              <n-flex :size="4" align="center">
                <inline-code :displayText="mod.steamWorkshopId.toString()"></inline-code>
                <JumpTo :url="`https://steamcommunity.com/sharedfiles/filedetails/?id=${mod.steamWorkshopId}`"></JumpTo>
              </n-flex>
            </n-descriptions-item>
            <!-- Expected Hash -->
            <n-descriptions-item :label="$t('modCard.expectedHash')">
              <inline-code :displayText="`${mod.expectedHash.slice(0,7)}...`" :src="mod.expectedHash"></inline-code>
            </n-descriptions-item>
            <!-- Home Directory -->
            <n-descriptions-item v-if="mod.homeDir" :label="$t('modCard.homeDir')">
              <n-flex :size="4" align="center">
                <inline-code :displayText="mod.homeDir"></inline-code>
                <reveal :path="mod.homeDir"></reveal>
              </n-flex>
            </n-descriptions-item>
          </n-descriptions>
        </n-flex>
      </template>
    </n-thing>
  </n-card>
</template>

<script lang="ts" setup>
import { CheckmarkCircleOutline, EllipseOutline, FolderOutline } from "@vicons/ionicons5";
import { enabled_mods } from "../../invokes.ts";
import type { BarotraumaMod } from "../../proto/mods.ts";
import InlineCode from "../utils/inlineCode.vue";
import JumpTo from "../utils/jumpTo.vue";
import Reveal from "../utils/Reveal.vue";

interface Props {
	mod: BarotraumaMod;
	index: number;
}

const is_enabled = (mod: BarotraumaMod): boolean => {
	return !!enabled_mods.value.find(
		(m) => m.steamWorkshopId === mod.steamWorkshopId,
	);
};

const props = defineProps<Props>();

type Emits = (e: "modSelected", data: BarotraumaMod) => void;

const emit = defineEmits<Emits>();

const onClick = () => {
	emit("modSelected", props.mod);
};
</script>

<style scoped>
.mod-card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.mod-status-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.mod-status-indicator.enabled {
  background: var(--color-success);
  box-shadow: 0 0 8px rgba(7, 202, 107, 0.4);
}

.mod-status-indicator.disabled {
  background: var(--color-warning);
  box-shadow: 0 0 8px rgba(232, 149, 88, 0.3);
}

.mod-card:hover .mod-status-indicator {
  transform: scale(1.2);
}

.mod-card-title {
  font-size: var(--text-base);
  font-weight: 600;
  transition: color 0.2s ease;
}
.mod-card:hover .mod-card-title {
  color: var(--color-warning);
}

.mod-badge {
  margin-left: auto;
}
</style>
