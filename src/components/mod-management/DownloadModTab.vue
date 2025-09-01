<template>
  <n-grid cols="2" style="height: 100%" x-gap="16" y-gap="8">
    <!-- Left Column: Input Form -->
    <n-gi>
      <n-card :title="$t('downloadMods.inputTitle')" embedded style="height: 100%">
        <n-input
            v-model:value="modInput"
            :autosize="{ minRows: 7, maxRows: 18 }"
            :placeholder="$t('downloadMods.modInputPlaceholder')"
            type="textarea"
        />
        <template #footer>
          <n-space>
            <n-button :loading="isAddingMods" type="primary" @click="addMods">
              {{ $t('downloadMods.addToQueue') }}
            </n-button>
            <n-button @click="clearInput">
              {{ $t('downloadMods.clear') }}
            </n-button>
          </n-space>
        </template>

      </n-card>
    </n-gi>

    <!-- Right Column: Mod Queue -->
    <n-gi>
      <n-card :title="$t('downloadMods.queueTitle')" embedded style="height: 100%">
        <template #header-extra>
          <n-tag size="small">
            {{ $t('downloadMods.queueCount', {count: modQueue.length}) }}
          </n-tag>
        </template>
        <template #default>

          <n-scrollbar style="height: 40vh">
            <n-list v-if="modQueue.length > 0">
              <n-list-item v-for="(mod, index) in modQueue" :key="index">

                <WorkshopItemDisplay :item="mod" style="padding: 10px"/>

                <template #suffix>
                  <n-button size="large" text @click="removeFromQueue(index)">
                    <n-icon size="2em">
                      <close-outline/>
                    </n-icon>
                  </n-button>
                </template>
              </n-list-item>
            </n-list>
            <!-- Empty state when no mods in queue -->
            <n-empty v-else :content="$t('downloadMods.empty')" style="margin-top: 20px"/>

          </n-scrollbar>
        </template>

        <template #footer>
          <n-space style="margin-top: 0.8em">
            <n-button :loading="isProcessing" type="success" @click="processQueue">
              {{ $t('downloadMods.processAll') }}
            </n-button>
            <n-button @click="clearQueue">
              {{ $t('downloadMods.clearQueue') }}
            </n-button>
          </n-space>
        </template>
      </n-card>


    </n-gi>
  </n-grid>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { CloseOutline } from "@vicons/ionicons5";
import { getSteamWorkshopId } from "../../composables/network.ts";
import {
	download_mods,
	get_workshop_items,
	install_mods,
	is_barotrauma_mod,
} from "../../invokes.ts";
import { useMessage } from "naive-ui";
import { WorkshopItem } from "../../proto/workshop.ts";
import { ModItem, ModStatus } from "../../composables/workshop.ts";
import WorkshopItemDisplay from "./WorkshopItemDisplay.vue";

// Initialize Naive UI message instance for user feedback
const message = useMessage();

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

	const mod: ModItem = { status: ModStatus.Pending };

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
		let isValid = false;
		let target = undefined;
		let retrieved_items: WorkshopItem[] = await get_workshop_items([mod.id]);

		if (retrieved_items.length > 0) {
			console.log("retrieved item info.");

			target = retrieved_items[0];

			isValid = await is_barotrauma_mod(target);
		}

		if (isValid) {
			mod.verified = true;
			mod.retrieved = target;
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
			mod.status = ModStatus.Error;
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
	const validCount = modQueue.value.filter(
		(m) => m.status !== ModStatus.Error,
	).length;
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
		.filter((mod) => mod.id && mod.status !== ModStatus.Error)
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
	status: ModStatus,
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
		updateModStatus(
			(mod) => !!mod.id && mod.status !== ModStatus.Error,
			ModStatus.Downloading,
		);

		// Call backend to download all mods
		await download_mods(modIds);
		await install_mods(modIds);
		// Mark successfully downloading mods as completed
		updateModStatus(
			(mod) => !!mod.id && mod.status === ModStatus.Downloading,
			ModStatus.Completed,
		);

		message.success(`Successfully downloaded ${modIds.length} mods.`);
	} catch (error) {
		console.error("Download failed:", error);
		message.error("Failed to download mods.");

		// Mark any downloading mods as failed
		updateModStatus(
			(mod) => !!mod.id && mod.status === ModStatus.Downloading,
			ModStatus.Error,
		);
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
</script>