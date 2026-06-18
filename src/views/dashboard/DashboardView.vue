<template>
  <TitledPage>

    <template #title>
      <n-h1 v-text="$t('navigation.dashboard')"></n-h1>
      <n-tag
        v-if="active_profile"
        type="success"
        style="margin-left: 12px; vertical-align: middle;"
      >
        {{ $t('dashboard.activeProfile') }}: {{ active_profile }}
      </n-tag>
    </template>

    <n-alert
      v-if="updatesAvailable.length > 0"
      type="info"
      :title="$t('dashboard.updatesAvailable')"
      style="margin-bottom: 16px"
      closable
      @close="updatesAvailable = []"
    >
      {{ $t('dashboard.updatesCount', { count: updatesAvailable.length }) }}
    </n-alert>

    <n-alert
      v-if="conflicts && conflicts.missingDependencies.length > 0"
      type="warning"
      :title="$t('dashboard.conflictsDetected')"
      style="margin-bottom: 16px"
    >
      <ul style="margin: 4px 0; padding-left: 20px">
        <li v-for="dep in conflicts.missingDependencies" :key="dep.modName + dep.dependencyName">
          <strong>{{ dep.modName }}</strong>
          {{ $t('dashboard.missingDependency') }}
          <strong>{{ dep.dependencyName }}</strong>
        </li>
      </ul>
    </n-alert>

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

      <n-grid-item span="5" style="height: 100%">
        <n-card :title="$t('modDetails.title')">
          <ModDetails :mod="curMod"></ModDetails>
        </n-card>
      </n-grid-item>
    </n-grid>
  </TitledPage>
</template>

<script lang="ts" setup>

import { active_profile, enabled_mods, installed_mod, check_workshop_updates } from "../../invokes.ts";
import { detect_mod_conflicts } from "../../invokes.ts";
import type { ConflictReport, WorkshopUpdateStatus } from "../../invokes.ts";
import TitledPage from "../../components/core/TitledPage.vue";
import { onMounted, ref } from "vue";
import ModDetails from "../../components/dashboard/ModDetails.vue";
import ModList from "../../components/dashboard/ModList.vue";
import type { BarotraumaMod } from "../../proto/mods.ts";
import RefreshMods from "../../components/utils/refreshMods.vue";
import { useThemeVars } from "naive-ui";

const curMod = ref<BarotraumaMod | null>(null);
const themeVars = useThemeVars();
const conflicts = ref<ConflictReport | null>(null);
const updatesAvailable = ref<WorkshopUpdateStatus[]>([]);

onMounted(async () => {
	try {
		conflicts.value = await detect_mod_conflicts();
	} catch {
		// Silently ignore — conflict detection is advisory
	}
	try {
		const allUpdates = await check_workshop_updates();
		updatesAvailable.value = allUpdates.filter((u) => u.hasUpdate);
	} catch {
		// Silently ignore — update check is advisory
	}
});

function handleModClick(mod: BarotraumaMod) {
	curMod.value = mod;
}
</script>