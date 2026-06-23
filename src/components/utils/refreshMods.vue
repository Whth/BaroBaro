<template>
  <n-button circle quaternary style="vertical-align: text-bottom" @click="refreshMods">
    <template #icon>
      <n-icon>
        <RefreshIcon/>
      </n-icon>
    </template>
  </n-button>
</template>

<script lang="ts" setup>
import { RefreshOutline as RefreshIcon } from "@vicons/ionicons5";
import {
	list_enabled_mods,
	list_installed_mods,
	list_mod_lists,
	retrieve_mod_metadata,
} from "../../invokes.ts";

async function refreshMods() {
	await Promise.all([
		list_installed_mods(),
		list_enabled_mods(),
		list_mod_lists(),
	]);
	await retrieve_mod_metadata();
}
</script>


