<template>
  <TitledPage>

    <template #title>
      <n-h1 v-text="$t('navigation.dashboard')"></n-h1>
    </template>

    <n-grid cols="12" x-gap="24" y-gap="24">
      <n-grid-item span="7">
        <n-card :title="$t('modList.title')">
          <template #header-extra>
            <RefreshMods/>
          </template>
          <n-flex>

            <n-progress
                :color="themeVars.warningColor"
                :percentage="enabled_mods.length / installed_mod.length * 100"
                :rail-color="themeVars.errorColor"
                indicator-placement="inside"
                type="line"
            >
              {{ enabled_mods.length }}/{{ installed_mod.length }}
            </n-progress>
            <ModList @viewing-mod="handleModClick"/>
          </n-flex>
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
import RefreshMods from "../../components/utils/refreshMods.vue";
import { enabled_mods, installed_mod } from "../../invokes.ts";
import { useThemeVars } from "naive-ui";

const curMod = ref<BarotraumaMod | null>(null);
const themeVars = useThemeVars();

function handleModClick(mod: BarotraumaMod) {
	console.log("Clicked mod:", mod);
	curMod.value = mod;
}
</script>