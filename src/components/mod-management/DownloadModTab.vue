<template>
  <n-card title="Download Mods">
    <n-form>
      <n-form-item label="Mod ID or Link">
        <n-input
            v-model:value="modInput"
            :autosize="{ minRows: 3, maxRows: 6 }"
            placeholder="Paste mod ID or link here (one per line for batch processing)"
            type=textarea
        />
      </n-form-item>
      <n-form-item>
        <n-space>
          <n-button :loading="isAddingMods" type="primary" @click="addMods">Add to Queue</n-button>
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
              {{ mod.id ? mod.id.toString() : mod.url }}
              <n-tag v-if="mod.verified" size="small" style="margin-left: 8px" type="success">
                Verified
              </n-tag>
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
        <n-button :loading="isProcessing" type="success" @click="processQueue">Process All</n-button>
        <n-button @click="clearQueue">Clear Queue</n-button>
      </n-space>
    </n-card>

    <n-empty v-else description="No mods in queue"/>
  </n-card>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { getSteamWorkshopId } from "../../composables/network.ts";
import { CloseOutline } from "@vicons/ionicons5";
import { download_mods, is_barotrauma_mod } from "../../invokes.ts";
import { useMessage } from "naive-ui";

const message = useMessage();

interface ModItem {
	id?: number;
	url?: string;
	status: "pending" | "downloading" | "completed" | "error";
	verified?: boolean;
}

const modInput = ref("");
const modQueue = ref<ModItem[]>([]);
// Add loading state
const isAddingMods = ref(false);
const isProcessing = ref(false);

// Parse a single input line into a mod item
const parseModInput = (input: string): ModItem => {
	const mod: ModItem = { status: "pending" };

	if (input.startsWith("http")) {
		mod.url = input;
		const extractedId = getSteamWorkshopId(input);
		if (extractedId !== null) {
			mod.id = extractedId;
		}
	} else {
		const parsedId = Number(input.trim());
		if (!isNaN(parsedId) && Number.isInteger(parsedId) && parsedId > 0) {
			mod.id = parsedId;
		}
	}

	return mod;
};

// Validate if a mod is a valid Barotrauma mod
const validateMod = async (mod: ModItem): Promise<boolean> => {
	if (!mod.id) return false;

	try {
		return await is_barotrauma_mod(mod.id);
	} catch (error) {
		console.error("Error validating mod:", error);
		return false;
	}
};

const addMods = async () => {
	// Set loading state
	isAddingMods.value = true;

	const inputs = modInput.value
		.split("\n")
		.filter((item) => item.trim() !== "");

	for (const input of inputs) {
		const mod = parseModInput(input);

		// Check if mod is already in queue
		const isDuplicate = modQueue.value.some((existingMod) => {
			// Compare by ID if available
			if (mod.id && existingMod.id) {
				return mod.id === existingMod.id;
			}
			// Otherwise compare by URL if available
			if (mod.url && existingMod.url) {
				return mod.url === existingMod.url;
			}
			// If neither ID nor URL match, not a duplicate
			return false;
		});

		// Skip if duplicate
		if (isDuplicate) {
			// Show message to user about duplication
			message.warning(
				`Mod "${mod.id ? mod.id : mod.url}" is already in the queue`,
			);
			continue;
		}

		// Only validate mods with IDs
		if (mod.id) {
			const isValid = await validateMod(mod);
			if (!isValid) {
				mod.status = "error";
			} else {
				// Add verification flag
				mod.verified = true;
			}
		}

		modQueue.value.push(mod);
	}

	modInput.value = "";
	// Reset loading state
	isAddingMods.value = false;
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
	// Set processing state
	isProcessing.value = true;

	// Get all valid mod IDs from the queue
	const modIds = modQueue.value
		.filter((mod) => mod.id && mod.status !== "error")
		.map((mod) => mod.id as number);

	if (modIds.length === 0) {
		isProcessing.value = false;
		return;
	}

	try {
		// Update status for all mods that will be downloaded
		modQueue.value
			.filter((mod) => mod.id && mod.status !== "error")
			.forEach((mod) => {
				mod.status = "downloading";
			});

		// Call the download_mods function with all mod IDs
		await download_mods(modIds);

		// Update status for all mods to completed
		modQueue.value
			.filter((mod) => mod.id && mod.status === "downloading")
			.forEach((mod) => {
				mod.status = "completed";
			});

		message.success(`Successfully downloaded ${modIds.length} mods`);
	} catch (error) {
		console.error("Error downloading mods:", error);
		message.error("Failed to download mods");

		// Update status for all mods to error
		modQueue.value
			.filter((mod) => mod.id && mod.status === "downloading")
			.forEach((mod) => {
				mod.status = "error";
			});
	} finally {
		isProcessing.value = false;
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