import { invoke } from "@tauri-apps/api/core";
import { computed, type Ref, ref } from "vue";
import { BuildInfo } from "./proto/build_info.ts";
import { Config } from "./proto/config";
import type { BarotraumaMod, ModList } from "./proto/mods";
import type { WorkshopItem } from "./proto/workshop.ts";

export const config: Ref<Config> = ref(Config.create());
export const installed_mod: Ref<BarotraumaMod[]> = ref([]);
export const mod_lists: Ref<ModList[]> = ref([]);

export const enabled_mods: Ref<BarotraumaMod[]> = ref([]);

export const BAROTRAUMA_GAME_ID: number = 602960;

export async function refresh_config() {
	config.value = await invoke("read_config").then((data) =>
		Config.fromJSON(data),
	);
}

export const active_profile = computed(() => config.value.activeProfile);

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
}

export async function download_mods(mods: number[]) {
	await invoke("download_mods", { mods });
}

export async function list_mod_lists() {
	mod_lists.value = await invoke("list_mod_lists");
}

export async function get_build_info(): Promise<BuildInfo> {
	return await invoke("get_build_info").then((data) =>
		BuildInfo.fromJSON(data),
	);
}

export async function list_enabled_mods() {
	enabled_mods.value = await invoke("list_enabled_mods");
}

// Retrieve metadata for enabled mods
export async function retrieve_mod_metadata() {
	await refresh_config();

	const retried = await invoke<BarotraumaMod[]>("retrieve_mod_metadata", {
		mods: installed_mod.value,
		batchSize: config.value.metadataRetrieveBatchsize,
	});

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

export async function is_barotrauma_mod(
	item: number | WorkshopItem,
): Promise<boolean> {
	if (typeof item === "number") {
		return await invoke("is_barotrauma_mod", { itemId: item });
	}
	return item.consumerAppId === BAROTRAUMA_GAME_ID;
}

export async function install_mods(modIds: number[]): Promise<null> {
	return await invoke("install_mods", { modIds });
}

export async function get_background_image(): Promise<string | null> {
	return await invoke("get_background_image");
}

export async function get_mod_occupation(modId: number): Promise<number> {
	return await invoke("get_mod_occupation", { modId });
}

export async function get_mod_hash(modId: number): Promise<string> {
	return await invoke("get_mod_hash", { modId });
}

/** Status for a single mod in an update check. */
export interface ModUpdateStatus {
	modId: number;
	needsUpdate: boolean;
	storedHash: string | null;
	currentHash: string | null;
}

export async function check_mod_updates(
	modIds: number[],
): Promise<ModUpdateStatus[]> {
	return await invoke("check_mod_updates", { modIds });
}

export async function get_workshop_items(
	itemIds: number[],
): Promise<WorkshopItem[]> {
	return await invoke("get_workshop_items", { itemIds });
}

export async function uninstall_mods(modIds: number[]): Promise<null> {
	return await invoke("uninstall_mods", { modIds });
}

export async function create_mod_list(profileName: string): Promise<ModList> {
	const result: ModList = await invoke("create_mod_list", { profileName });
	await list_mod_lists();
	return result;
}

export async function delete_mod_list(profileName: string): Promise<void> {
	await invoke("delete_mod_list", { profileName });
	await list_mod_lists();
	await refresh_config();
}

export async function apply_mod_list(profileName: string): Promise<void> {
	await invoke("apply_mod_list", { profileName });
	await list_installed_mods();
	await list_enabled_mods();
	await refresh_config();
}

export async function set_active_profile(name: string): Promise<void> {
	await invoke("set_active_profile", { profileName: name });
	await refresh_config();
}

export async function clear_active_profile(): Promise<void> {
	await invoke("clear_active_profile");
	await refresh_config();
}

export async function reorder_enabled_mods(
	orderedIds: number[],
): Promise<void> {
	await invoke("reorder_enabled_mods", { orderedIds });
}

/** Result of comparing two mod profiles. */
export interface ProfileDiff {
	onlyInA: string[];
	onlyInB: string[];
	inBoth: string[];
}

export async function rename_profile(
	oldName: string,
	newName: string,
): Promise<ModList> {
	const result = (await invoke("rename_profile", {
		oldName,
		newName,
	})) as ModList;
	await list_mod_lists();
	await refresh_config();
	return result;
}

export async function compare_profiles(
	nameA: string,
	nameB: string,
): Promise<ProfileDiff> {
	return await invoke("compare_profiles", { nameA, nameB });
}

export async function export_profile(
	profileName: string,
	exportPath: string,
): Promise<void> {
	await invoke("export_profile", { profileName, exportPath });
}

export async function import_profile(path: string): Promise<ModList> {
	const result = (await invoke("import_profile", { path })) as ModList;
	await list_mod_lists();
	return result;
}

/** A dependency declared by a mod that is not satisfied by any enabled mod. */
export interface MissingDependency {
	modName: string;
	modSteamId: number;
	dependencyName: string;
	dependencySteamId: number | null;
}

/** Result of conflict detection among enabled mods. */
export interface ConflictReport {
	missingDependencies: MissingDependency[];
}

/** Detects missing dependencies among currently enabled mods. */
export async function detect_mod_conflicts(): Promise<ConflictReport> {
	return await invoke("detect_mod_conflicts");
}

/** Status of a single mod's workshop update check. */
export interface WorkshopUpdateStatus {
	modId: number;
	modName: string;
	hasUpdate: boolean;
	localLastModified: number | null;
	workshopLastUpdated: number | null;
}

/** Network connectivity status. */
export interface NetworkStatus {
	steamApi: boolean;
	steamcmdAvailable: boolean;
}

/** Checks installed mods against Steam Workshop for available updates. */
export async function check_workshop_updates(): Promise<
	WorkshopUpdateStatus[]
> {
	return await invoke("check_workshop_updates");
}

/** Checks network connectivity to Steam services. */
export async function check_network_status(): Promise<NetworkStatus> {
	return await invoke("check_network_status");
}

/** Returns popular Barotrauma mods from the Steam Workshop. */
export async function get_popular_mods(): Promise<WorkshopItem[]> {
	return await invoke("get_popular_mods");
}
