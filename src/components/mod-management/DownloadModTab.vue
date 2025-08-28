<template>
  <n-card title="Download Mods">
    <n-form>
      <n-form-item label="Mod ID or Link">
        <n-input
            v-model:value="modInput"
            :autosize="{ minRows: 3, maxRows: 6 }"
            placeholder="Paste mod ID or link here (one per line for batch processing)"
            type="textarea"
        />
      </n-form-item>
      <n-form-item>
        <n-space>
          <n-button type="primary" @click="addMods">Add to Queue</n-button>
          <n-button @click="clearInput">Clear</n-button>
        </n-space>
      </n-form-item>
    </n-form>

    <n-divider/>

    <n-card v-if="modQueue.length > 0" title="Download Queue">
      <n-list>
        <n-list-item v-for="(mod, index) in modQueue" :key="index">
          <n-thing>
            <template #header>
              {{ mod.id || mod.url }}
            </template>
            <template #description>
              <n-tag :type="getStatusType(mod.status)">
                {{ mod.status }}
              </n-tag>
            </template>
          </n-thing>
          <template #suffix>
            <n-button text @click="removeFromQueue(index)">
              <n-icon>
                <close-outline/>
              </n-icon>
            </n-button>
          </template>
        </n-list-item>
      </n-list>
      <n-space style="margin-top: 16px">
        <n-button type="success" @click="processQueue">Process All</n-button>
        <n-button @click="clearQueue">Clear Queue</n-button>
      </n-space>
    </n-card>

    <n-empty v-else description="No mods in queue"/>
  </n-card>
</template>

<script lang="ts" setup>
import { ref } from "vue";

import { CloseOutline } from "@vicons/ionicons5";

interface ModItem {
	id?: string;
	url?: string;
	status: "pending" | "downloading" | "completed" | "error";
}

const modInput = ref("");
const modQueue = ref<ModItem[]>([]);

const addMods = () => {
	const inputs = modInput.value
		.split("\n")
		.filter((item) => item.trim() !== "");

	inputs.forEach((input) => {
		const mod: ModItem = {
			status: "pending",
		};

		if (input.startsWith("http")) {
			mod.url = input;
		} else {
			mod.id = input;
		}

		modQueue.value.push(mod);
	});

	modInput.value = "";
};

const clearInput = () => {
	modInput.value = "";
};

const removeFromQueue = (index: number) => {
	modQueue.value.splice(index, 1);
};

const clearQueue = () => {
	modQueue.value = [];
};

const processQueue = async () => {
	for (const mod of modQueue.value) {
		mod.status = "downloading";
		// Simulate download process
		await new Promise((resolve) => setTimeout(resolve, 1000));
		mod.status = "completed";
	}
};

const getStatusType = (status: ModItem["status"]) => {
	switch (status) {
		case "pending":
			return "default";
		case "downloading":
			return "warning";
		case "completed":
			return "success";
		case "error":
			return "error";
		default:
			return "default";
	}
};
</script>