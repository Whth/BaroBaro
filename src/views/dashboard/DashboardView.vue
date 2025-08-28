<template>
  <TitledPage>
    <template #title>
      <n-h1 v-text="$t('navigation.dashboard')"></n-h1>
    </template>

    <n-grid cols="12" x-gap="24" y-gap="24">
      <n-grid-item span="7">
        <n-card :title="$t('modList.title')">
          <template #header-extra>
            <n-button circle quaternary @click="refreshMods">
              <template #icon>
                <n-icon>
                  <RefreshIcon/>
                </n-icon>
              </template>
            </n-button>
          </template>
          <ModList @viewing-mod="handleModClick"></ModList>
        </n-card>
      </n-grid-item>

      <n-grid-item span="5">
        <n-card :title="$t('modDetails.title')">
          <ModDetails :mod="curMod"></ModDetails>
        </n-card>
      </n-grid-item>
    </n-grid>
  </TitledPage>
</template>

<script lang="ts" setup>
import TitledPage from "../../components/core/TitledPage.vue";
import { ref } from "vue";
import ModDetails from "../../components/dashboard/ModDetails.vue";
import ModList from "../../components/dashboard/ModList.vue";
import type { BarotraumaMod } from "../../proto/mods.ts";
import {
	list_enabled_mods,
	list_installed_mods,
	list_mod_lists,
	retrieve_mod_metadata,
} from "../../invokes.ts";
import { RefreshOutline as RefreshIcon } from "@vicons/ionicons5";

const curMod = ref<BarotraumaMod | null>(null);

async function refreshMods() {
	await Promise.all([
		list_installed_mods(),
		list_enabled_mods(),
		list_mod_lists(),
	]);
	await retrieve_mod_metadata();
}

function handleModClick(mod: BarotraumaMod) {
	console.log("Clicked mod:", mod);
	curMod.value = mod;
}
</script>