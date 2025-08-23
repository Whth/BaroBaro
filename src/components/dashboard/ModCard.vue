<template>
  <n-card
      :class="[
      'mod-card',
      { 'mod-card-disabled': !isEnabled, 'drag-over': isDragOver }
    ]"
      :draggable="draggable"
      :segmented="true"
      @dragend="$emit('dragend')"
      @dragenter="$emit('dragenter')"
      @dragleave="$emit('dragleave')"
      @dragover="$emit('dragover', $event)"
      @dragstart="$emit('dragstart', $event)"
      @drop="$emit('drop', $event)"
  >
    <template #header>
      <n-flex align="center" justify="space-between">
        <n-flex :size="12" align="center">
          <n-tag round size="small" type="primary">{{ index + 1 }}</n-tag>
          <n-ellipsis style="max-width: 200px">
            <strong>{{ mod.name }}</strong>
          </n-ellipsis>
        </n-flex>
        <n-button
            :type="isEnabled ? 'success' : 'default'"
            size="small"
            @click="toggleMod"
        >
          {{ isEnabled ? "Enabled" : "Disabled" }}
        </n-button>
      </n-flex>
    </template>

    <n-space :size="4" vertical>
      <n-text depth="3">Version: {{ mod.modVersion }}</n-text>
      <n-text depth="3">Steam ID: {{ mod.steamWorkshopId }}</n-text>
      <n-text depth="3">Game Version: {{ mod.gameVersion }}</n-text>
    </n-space>

    <template #footer>
      <n-flex align="center" justify="space-between">
        <n-tag :type="mod.corePackage ? 'success' : 'warning'" size="small">
          {{ mod.corePackage ? "Core Package" : "Mod" }}
        </n-tag>
        <n-button text type="primary" @click="selectMod">
          View Details
        </n-button>
      </n-flex>
    </template>
  </n-card>
</template>

<script lang="ts" setup>
import {defineEmits, defineProps} from "vue";
import type {BarotraumaMod} from "../../proto/mods";

const props = defineProps<{
  mod: BarotraumaMod;
  index: number;
  draggable?: boolean;
  isDragOver?: boolean;
  isEnabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "toggleMod", id: string): void;
  (e: "selectMod", id: string): void;
  (e: "dragstart", event: DragEvent): void;
  (e: "dragover", event: DragEvent): void;
  (e: "dragenter"): void;
  (e: "dragleave"): void;
  (e: "drop", event: DragEvent): void;
  (e: "dragend"): void;
}>();

const toggleMod = () => {
  emit("toggleMod", props.mod.steamWorkshopId);
};

const selectMod = () => {
  emit("selectMod", props.mod.steamWorkshopId);
};
</script>

<style scoped>
.mod-card {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.mod-card:hover {
  transform: translateY(-2px);
}

.mod-card-disabled {
  opacity: 0.6;
}

.drag-over {
  border-color: var(--n-border-color);
  box-shadow: 0 0 0 2px var(--n-color-primary);
}
</style>