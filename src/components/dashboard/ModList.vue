<template>
  <n-card :bordered="false" class="mod-list-card">
    <n-h2>Installed Mods</n-h2>

    <div
        class="mod-list-container"
        @dragover="handleDragOver"
    >
      <ModCard
          v-for="(mod, index) in installed_mod"
          :key="mod.steamWorkshopId"
          :draggable="true"
          :index="index"
          :is-drag-over="dragOverIndex === index"
          :is-enabled="isModEnabled(mod.steamWorkshopId, currentModList)"
          :mod="mod"
          @dragend="handleDragEnd"
          @dragenter="handleDragEnter(index)"
          @dragleave="handleDragLeave"
          @dragstart="handleDragStart(index, $event)"
          @drop="handleDrop(index, $event)"
          @toggle-mod="toggleMod"
          @select-mod="selectMod"
      />
    </div>

    <div class="mod-actions">
      <n-space>
        <n-button type="primary" @click="saveModOrder">
          Save Mod Order
        </n-button>
        <n-button @click="loadModOrder">
          Load Mod Order
        </n-button>
      </n-space>
    </div>
  </n-card>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import {
	installed_mod,
	isModEnabled,
	mod_lists,
} from "../../composables/useModManager";
import ModCard from "./ModCard.vue";

// Drag and drop state
const draggedItemIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);

// 当前mod列表
const currentModList = mod_lists.value.length > 0 ? mod_lists.value[0] : null;

// 事件处理
const emit = defineEmits<{
	(e: "toggle-mod", modId: string): void;
	(e: "select-mod", modId: string): void;
}>();

const toggleMod = (modId: string) => {
	emit("toggle-mod", modId);
};

const selectMod = (modId: string) => {
	emit("select-mod", modId);
};

// 拖拽相关方法
const handleDragStart = (index: number, event: DragEvent) => {
	draggedItemIndex.value = index;
	event.dataTransfer?.setData("text/plain", index.toString());
	event.dataTransfer!.effectAllowed = "move";
};

const handleDragOver = (event: DragEvent) => {
	event.preventDefault();
	event.dataTransfer!.dropEffect = "move";
};

const handleDragEnter = (index: number) => {
	dragOverIndex.value = index;
};

const handleDragLeave = () => {
	dragOverIndex.value = null;
};

const handleDrop = (index: number, event: DragEvent) => {
	event.preventDefault();

	if (draggedItemIndex.value !== null) {
		// 这里应该实现实际的重新排序逻辑
		console.log(`Move item from ${draggedItemIndex.value} to ${index}`);
	}

	resetDragState();
};

const handleDragEnd = () => {
	resetDragState();
};

const resetDragState = () => {
	draggedItemIndex.value = null;
	dragOverIndex.value = null;
};

// 保存和加载mod顺序
const saveModOrder = () => {
	// 实际实现应该调用后端API
	console.log("Saving mod order");
	window.$message?.success("Mod order saved!");
};

const loadModOrder = () => {
	// 实际实现应该调用后端API
	console.log("Loading mod order");
	window.$message?.info("Mod order loaded!");
};
</script>

<style scoped>
.mod-list-card {
  height: 100%;
}

.mod-list-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin: 20px 0;
}

.mod-actions {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid #eee;
}
</style>