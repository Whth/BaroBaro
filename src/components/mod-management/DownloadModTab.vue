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
import {
	download_mods,
	install_mods,
	is_barotrauma_mod,
} from "../../invokes.ts";
import { useMessage } from "naive-ui";

// Initialize Naive UI message instance for user feedback
const message = useMessage();

// Interface defining the structure of a mod item in the queue
interface ModItem {
	id?: number; // Steam Workshop ID (if available)
	url?: string; // Original URL input (if provided)
	status: "pending" | "downloading" | "completed" | "error"; // Current download status
	verified?: boolean; // Whether the mod has passed validation
}

// Reactive reference for user input (mod IDs or URLs)
const modInput = ref<string>("");

// Reactive list representing the current download queue
const modQueue = ref<ModItem[]>([]);

// Loading states for UI feedback
const isAddingMods = ref<boolean>(false); // True while parsing and adding mods
const isProcessing = ref<boolean>(false); // True while downloading mods

/**
 * Parses a single line of user input into a ModItem object.
 * Supports both direct IDs (e.g. 123456789) and URLs (e.g. https://steamcommunity.com/sharedfiles/filedetails/?id=123456789)
 *
 * @param line - The input string to parse
 * @returns Parsed ModItem or null if input is invalid/empty
 */
const parseInputLine = (line: string): ModItem | null => {
	const trimmed = line.trim();
	if (!trimmed) return null;

	const mod: ModItem = { status: "pending" };

	if (trimmed.startsWith("http")) {
		mod.url = trimmed;
		const id = getSteamWorkshopId(trimmed);
		if (id !== null) mod.id = id;
	} else {
		const num = Number(trimmed);
		if (!isNaN(num) && Number.isInteger(num) && num > 0) {
			mod.id = num;
		}
	}

	return mod;
};

/**
 * Checks if a given mod is already present in the queue.
 * Uses ID comparison first, falls back to URL if available.
 *
 * @param mod - The mod to check for duplication
 * @returns True if the mod already exists in the queue
 */
const isDuplicateMod = (mod: ModItem): boolean => {
	return modQueue.value.some((existing) => {
		if (mod.id && existing.id) return mod.id === existing.id;
		if (mod.url && existing.url) return mod.url === existing.url;
		return false;
	});
};

/**
 * Validates whether the given mod is a valid Barotrauma workshop mod.
 * Sets `verified` flag if valid.
 *
 * @param mod - The mod item to validate
 * @returns Promise resolving to true if valid, false otherwise
 */
const verifyMod = async (mod: ModItem): Promise<boolean> => {
	if (!mod.id) return false;

	try {
		const isValid = await is_barotrauma_mod(mod.id);
		if (isValid) {
			mod.verified = true;
		}
		return isValid;
	} catch (error) {
		console.error(`Failed to validate mod ${mod.id}:`, error);
		return false;
	}
};

/**
 * Shows a warning message when a duplicate mod is added.
 *
 * @param mod - The duplicate mod to display in the message
 */
const showDuplicateWarning = (mod: ModItem) => {
	message.warning(
		`Mod "${mod.id ? `ID: ${mod.id}` : mod.url}" is already in the queue`,
	);
};

/**
 * Shows an error message when a mod fails validation.
 *
 * @param mod - The invalid mod to display in the message
 */
const showInvalidModWarning = (mod: ModItem) => {
	message.error(
		`Mod "${mod.id ? `ID: ${mod.id}` : mod.url}" is not a valid Barotrauma mod`,
	);
};

/**
 * Attempts to add a single mod (from one input line) to the queue.
 * Handles parsing, deduplication, and validation.
 *
 * @param input - A single line of user input
 */
const addSingleModToQueue = async (input: string): Promise<void> => {
	const mod = parseInputLine(input);
	if (!mod) return;

	if (isDuplicateMod(mod)) {
		showDuplicateWarning(mod);
		return;
	}

	// Only verify mods that have an ID
	if (mod.id) {
		const isValid = await verifyMod(mod);
		if (!isValid) {
			mod.status = "error";
			showInvalidModWarning(mod);
		}
	}

	modQueue.value.push(mod);
};

/**
 * Adds all non-empty lines from the input field to the download queue.
 * Processes each line individually, validates, and provides user feedback.
 */
const addMods = async () => {
	isAddingMods.value = true;
	const lines = modInput.value.split("\n").filter((line) => line.trim() !== "");

	if (lines.length === 0) {
		message.info("No input provided.");
		isAddingMods.value = false;
		return;
	}

	for (const line of lines) {
		await addSingleModToQueue(line);
	}

	modInput.value = "";
	const validCount = modQueue.value.filter((m) => m.status !== "error").length;
	message.success(`Added ${validCount} mods to queue.`);
	isAddingMods.value = false;
};

/**
 * Clears the input textarea.
 */
const clearInput = () => {
	modInput.value = "";
};

/**
 * Removes a mod from the queue by index.
 *
 * @param index - Index of the mod in the queue array
 */
const removeFromQueue = (index: number) => {
	modQueue.value.splice(index, 1);
};

/**
 * Clears the entire download queue.
 */
const clearQueue = () => {
	modQueue.value = [];
};

/**
 * Extracts all valid mod IDs from the queue (those with ID and not in error state).
 *
 * @returns Array of mod IDs ready for download
 */
const getValidModIds = (): number[] => {
	return modQueue.value
		.filter((mod) => mod.id && mod.status !== "error")
		.map((mod) => mod.id as number);
};

/**
 * Updates the status of multiple mods that match a condition.
 *
 * @param predicate - Function to test each mod
 * @param status - New status to set
 */
const updateModStatus = (
	predicate: (mod: ModItem) => boolean,
	status: ModItem["status"],
) => {
	modQueue.value.forEach((mod) => {
		if (predicate(mod)) {
			mod.status = status;
		}
	});
};

/**
 * Starts processing the entire download queue.
 * Downloads all valid mods in one batch and updates their status accordingly.
 */
const processQueue = async () => {
	const modIds = getValidModIds();

	if (modIds.length === 0) {
		message.warning("No valid mods to download.");
		return;
	}

	isProcessing.value = true;

	try {
		// Mark all valid mods as downloading
		updateModStatus((mod) => !!mod.id && mod.status !== "error", "downloading");

		// Call backend to download all mods
		await download_mods(modIds);
		await install_mods(modIds);
		// Mark successfully downloading mods as completed
		updateModStatus(
			(mod) => !!mod.id && mod.status === "downloading",
			"completed",
		);

		message.success(`Successfully downloaded ${modIds.length} mods.`);
	} catch (error) {
		console.error("Download failed:", error);
		message.error("Failed to download mods.");

		// Mark any downloading mods as failed
		updateModStatus((mod) => !!mod.id && mod.status === "downloading", "error");
	} finally {
		isProcessing.value = false;
	}
};

/**
 * Maps a mod's status string to a Naive UI tag type for visual styling.
 *
 * @param status - The current status of the mod
 * @returns Corresponding tag type
 */
const getStatusType = (
	status: ModItem["status"],
): "default" | "warning" | "success" | "error" => {
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