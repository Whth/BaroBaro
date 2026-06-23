<template>
  <TitledPage>

    <template #title>
      <n-h1 v-text="$t('navigation.dashboard')"></n-h1>
      <n-select
          :options="profileOptions"
          :placeholder="$t('dashboard.selectProfile')"
          :value="active_profile ?? null"
          clearable
          size="small"
          style="margin-left: 12px; vertical-align: middle; min-width: 180px;"
          @update:value="handleProfileChange"
      />
    </template>

    <n-alert
        v-if="updatesAvailable.length > 0"
        :title="$t('dashboard.updatesAvailable')"
        closable
        style="margin-bottom: 16px"
        type="info"
        @close="updatesAvailable = []"
    >
      {{ $t('dashboard.updatesCount', {count: updatesAvailable.length}) }}
    </n-alert>

    <n-alert
        v-if="conflicts && conflicts.missingDependencies.length > 0"
        :title="$t('dashboard.conflictsDetected')"
        style="margin-bottom: 16px"
        type="warning"
    >
      <ul style="margin: 4px 0; padding-left: 20px">
        <li v-for="dep in conflicts.missingDependencies" :key="dep.modName + dep.dependencyName">
          <strong>{{ dep.modName }}</strong>
          {{ $t('dashboard.missingDependency') }}
          <strong>{{ dep.dependencyName }}</strong>
        </li>
      </ul>
    </n-alert>
    <n-alert
        v-if="duplicateMods.length > 0"
        :title="$t('dashboard.duplicatesFound')"
        closable
        style="margin-bottom: 16px"
        type="warning"
    >
      <p>{{ $t('dashboard.duplicatesDescription', {count: duplicateMods.length}) }}</p>
      <ul style="margin: 4px 0; padding-left: 20px">
        <li v-for="dup in duplicateMods" :key="dup.name">
          <strong>{{ dup.name }}</strong> (×{{ dup.count }})
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
            <n-scrollbar style="max-height: 60vh">
              <ModList @viewing-mod="handleModClick"/>
            </n-scrollbar>
          </n-flex>
        </n-card>
      </n-grid-item>

      <n-grid-item span="5">
        <n-card :title="$t('modDetails.title')">
          <n-scrollbar style="max-height: 70vh">
            <ModDetails :mod="curMod"></ModDetails>
          </n-scrollbar>
        </n-card>
      </n-grid-item>
    </n-grid>
  </TitledPage>
</template>

<script lang="ts" setup>
import { useThemeVars } from "naive-ui";
import { computed, onMounted, ref } from "vue";
import TitledPage from "../../components/core/TitledPage.vue";
import ModDetails from "../../components/dashboard/ModDetails.vue";
import ModList from "../../components/dashboard/ModList.vue";
import RefreshMods from "../../components/utils/refreshMods.vue";
import type { ConflictReport, WorkshopUpdateStatus } from "../../invokes.ts";
import {
	active_profile,
	check_workshop_updates,
	clear_active_profile,
	detect_mod_conflicts,
	enabled_mods,
	installed_mod,
	mod_lists,
	set_active_profile,
} from "../../invokes.ts";
import type { BarotraumaMod } from "../../proto/mods.ts";

const curMod = ref<BarotraumaMod | null>(null);
const themeVars = useThemeVars();
const conflicts = ref<ConflictReport | null>(null);
const updatesAvailable = ref<WorkshopUpdateStatus[]>([]);

const profileOptions = computed(() =>
	mod_lists.value.map((list) => ({
		label: list.profileName,
		value: list.profileName,
	})),
);

async function handleProfileChange(value: string | null) {
	if (value) {
		await set_active_profile(value);
	} else {
		await clear_active_profile();
	}
}

const duplicateMods = computed(() => {
	const seen = new Map<string, { name: string; count: number }>();
	for (const mod of enabled_mods.value) {
		const key =
			mod.steamWorkshopId > 0
				? `ws:${mod.steamWorkshopId}`
				: `name:${mod.name}`;
		const existing = seen.get(key);
		if (existing) {
			existing.count++;
		} else {
			seen.set(key, { name: mod.name, count: 1 });
		}
	}
	return [...seen.values()].filter((entry) => entry.count > 1);
});

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