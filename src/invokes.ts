import {invoke} from "@tauri-apps/api/core";
import {Config} from "./proto/config";
import {Ref, ref} from "vue";
import {BarotraumaMod, ModList} from "./proto/mods";


export const config = ref(Config.create())
export const installed_mod: Ref<BarotraumaMod[]> = ref([])

export const mod_lists: Ref<ModList[]> = ref([])


export async function refresh_config() {
    config.value = await invoke("read_config").then((data) => Config.fromJSON(data));
}

export async function save_config() {
    await invoke("write_config", {config: config.value});
}

export async function list_installed_mods() {
    installed_mod.value = await invoke("list_installed_mods");
}

export async function download_mod(mods: number[]) {
    await invoke("download_mod", {mods});
}

export async function list_mod_lists() {
    mod_lists.value = await invoke("list_mod_lists");
}