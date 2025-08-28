import { invoke } from "@tauri-apps/api/core";
import { Config } from "./proto/config";
import { type Ref, ref } from "vue";
import type { BarotraumaMod, ModList } from "./proto/mods";
import { BuildInfo } from "./proto/build_info.ts";

export const config: Ref<Config> = ref(Config.create());
export const installed_mod: Ref<BarotraumaMod[]> = ref([]);
export const mod_lists: Ref<ModList[]> = ref([]);

export const enabled_mods: Ref<BarotraumaMod[]> = ref([]);

export async function refresh_config() {
	config.value = await invoke("read_config").then((data) =>
		Config.fromJSON(data),
	);
}

export async function save_config() {
	await invoke("write_config", { config: config.value });
}

export async function reset_config() {
	config.value = await invoke("get_default_config").then((data) =>
		Config.fromJSON(data),
	);
}

export async function list_installed_mods() {
	installed_mod.value = await invoke("list_installed_mods");
	console.log(`Found ${installed_mod.value.length} installed mods`);
}

export async function download_mods(mods: number[]) {
	await invoke("download_mods", { mods });
}

export async function list_mod_lists() {
	mod_lists.value = await invoke("list_mod_lists");
	console.log(`Found ${mod_lists.value.length} mod lists`);
}

export async function get_build_info(): Promise<BuildInfo> {
	return await invoke("get_build_info").then((data) =>
		BuildInfo.fromJSON(data),
	);
}

export async function list_enabled_mods() {
	enabled_mods.value = await invoke("list_enabled_mods");
	console.log(`Found ${enabled_mods.value.length} enabled mods`);
}

// Retrieve metadata for enabled mods
export async function retrieve_mod_metadata() {
	await refresh_config();
	console.log(`Retrieving metadata for ${installed_mod.value.length} mods`);

	let retried = await invoke<BarotraumaMod[]>("retrieve_mod_metadata", {
		mods: installed_mod.value,
		batchSize: config.value.metadataRetrieveBatchsize,
	});

	console.log(`Retrieved metadata for ${retried.length} mods`);

	const enabled_mapping = new Map(
		enabled_mods.value.map((mod) => [mod.steamWorkshopId, mod]),
	);

	const installed_mapping = new Map(
		installed_mod.value.map((mod) => [mod.steamWorkshopId, mod]),
	);

	retried.forEach((mod) => {
		const enabledMod = enabled_mapping.get(mod.steamWorkshopId);
		const installedMod = installed_mapping.get(mod.steamWorkshopId);
		if (enabledMod !== undefined) {
			Object.assign(enabledMod, mod);
		}
		if (installedMod !== undefined) {
			Object.assign(installedMod, mod);
		}
	});
}

export async function is_barotrauma_mod(itemId: number): Promise<boolean> {
	return await invoke("is_barotrauma_mod", { itemId });
}
