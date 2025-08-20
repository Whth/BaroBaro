import {invoke} from "@tauri-apps/api/core";
import {Config} from "../proto/config";
import {Ref, ref} from "vue";
import {BarotraumaMod, ModList} from "../proto/mods";


const config = ref(Config.create())

const installed_mod: Ref<BarotraumaMod[]> = ref([])

const mod_lists: Ref<ModList[]> = ref([])


export async function refresh_config() {
    config.value = await invoke("read_config").then((data) => Config.fromJSON(data));
}

export async function save_config() {
    await invoke("write_config", {config: Config.toJSON(config.value)});
}

export async function list_installed_mods() {
    installed_mod.value = await invoke("list_installed_mods");
}

export async function download_mod(mods: number[]) {
    await invoke("download_mod", {mods});
}

export async function list_mod_lists() {

}