<template>
  <div class="profiles-container">
    <!-- Save current mods as profile -->
    <div class="profiles-header">
      <n-button type="primary" @click="showCreateDialog = true">
        {{ $t('profiles.saveCurrent') }}
      </n-button>
      <n-button size="small" style="margin-left: 8px;" @click="handleImport">
        {{ $t('profiles.import') }}
      </n-button>
      <n-button size="small" style="margin-left: 4px;" @click="handleExport">
        {{ $t('profiles.export') }}
      </n-button>
      <n-button
          v-if="active_profile"
          size="small"
          style="margin-left: 8px;"
          type="warning"
          @click="handleClearActive"
      >
        {{ $t('profiles.clearActive') }} ({{ active_profile }})
      </n-button>
    </div>

    <!-- Empty state -->
    <n-empty v-if="mod_lists.length === 0" :description="$t('profiles.noProfiles')" style="margin-top: 40px">
      <template #extra>
        <n-button type="primary" @click="showCreateDialog = true">
          {{ $t('profiles.saveCurrent') }}
        </n-button>
      </template>
    </n-empty>

    <!-- Profile list -->
    <div v-else class="profiles-grid">
      <n-card
          v-for="profile in mod_lists"
          :key="profile.profileName"
          :title="profile.profileName"
          hoverable
          size="small"
      >
        <template #header-extra>
          <n-tag size="small" type="info">
            {{ $t('profiles.modCount', {count: profile.mods.length}) }}
          </n-tag>
        </template>

        <div class="profile-info">
          <span class="profile-base">
            {{ $t('profiles.basePackage') }}: {{ profile.basePackage }}
          </span>
        </div>

        <template #action>
          <n-space justify="end">
            <n-button size="small" @click="promptRename(profile.profileName)">
              {{ $t('profiles.rename') }}
            </n-button>
            <n-button size="small" @click="promptCompare(profile.profileName)">
              {{ $t('profiles.compare') }}
            </n-button>
            <n-button size="small" type="primary" @click="handleApply(profile.profileName)">
              {{ $t('profiles.apply') }}
            </n-button>
            <n-button size="small" type="error" @click="promptDelete(profile.profileName)">
              {{ $t('app.delete') }}
            </n-button>
          </n-space>
        </template>
      </n-card>
    </div>
  </div>
  <!-- Create profile dialog -->
  <n-modal v-model:show="showCreateDialog" :title="$t('profiles.saveCurrent')" preset="dialog">
    <n-input
        v-model:value="newProfileName"
        :placeholder="$t('profiles.profileNamePlaceholder')"
        @keyup.enter="handleCreate"
    />
    <template #action>
      <n-button ghost @click="showCreateDialog = false">{{ $t('app.cancel') }}</n-button>
      <n-button :disabled="!newProfileName.trim()" :loading="isCreating" type="primary" @click="handleCreate">
        {{ $t('app.create') }}
      </n-button>
    </template>
  </n-modal>

  <!-- Apply confirmation dialog -->
  <n-modal v-model:show="showApplyDialog" :title="$t('profiles.apply')" preset="dialog">
    <p>{{ $t('profiles.applyConfirm', {name: targetProfile}) }}</p>
    <template #action>
      <n-button ghost @click="showApplyDialog = false">{{ $t('app.cancel') }}</n-button>
      <n-button :loading="isApplying" type="primary" @click="confirmApply">
        {{ $t('profiles.apply') }}
      </n-button>
    </template>
  </n-modal>

  <!-- Delete confirmation dialog -->
  <n-modal v-model:show="showDeleteDialog" :title="$t('app.delete')" preset="dialog">
    <p>{{ $t('profiles.deleteConfirm', {name: targetProfile}) }}</p>
    <template #action>
      <n-button ghost @click="showDeleteDialog = false">{{ $t('app.cancel') }}</n-button>
      <n-button :loading="isDeleting" type="error" @click="confirmDelete">
        {{ $t('app.delete') }}
      </n-button>
    </template>
  </n-modal>

  <!-- Rename profile dialog -->
  <n-modal v-model:show="showRenameDialog" :title="$t('profiles.renameTitle')" preset="dialog">
    <n-input
        v-model:value="newProfileName"
        :placeholder="$t('profiles.renamePlaceholder')"
        @keyup.enter="confirmRename"
    />
    <template #action>
      <n-button ghost @click="showRenameDialog = false">{{ $t('app.cancel') }}</n-button>
      <n-button :disabled="!newProfileName.trim()" :loading="isRenaming" type="primary" @click="confirmRename">
        {{ $t('profiles.rename') }}
      </n-button>
    </template>
  </n-modal>

  <!-- Compare profiles dialog -->
  <n-modal v-model:show="showCompareDialog" :title="$t('profiles.compareTitle')" preset="dialog" style="width: 600px;">
    <div style="display: flex; gap: 12px; margin-bottom: 12px;">
      <n-select
          v-model:value="compareProfileA"
          :options="profileOptions"
          :placeholder="$t('profiles.selectProfileA')"
          style="flex: 1;"
          @update:value="clearDiff"
      />
      <n-select
          v-model:value="compareProfileB"
          :options="profileOptions"
          :placeholder="$t('profiles.selectProfileB')"
          style="flex: 1;"
          @update:value="clearDiff"
      />
    </div>
    <n-button
        :disabled="!compareProfileA || !compareProfileB || compareProfileA === compareProfileB"
        :loading="isComparing"
        block
        type="primary"
        @click="confirmCompare"
    >
      {{ $t('profiles.compare') }}
    </n-button>
    <div v-if="diffResult" style="margin-top: 12px;">
      <div v-if="diffResult.inBoth.length > 0" style="margin-bottom: 8px;">
        <n-text strong>{{ $t('profiles.inBoth') }} ({{ diffResult.inBoth.length }}):</n-text>
        <n-tag v-for="mod in diffResult.inBoth" :key="mod" size="small" style="margin: 2px;">{{ mod }}</n-tag>
      </div>
      <div v-if="diffResult.onlyInA.length > 0" style="margin-bottom: 8px;">
        <n-text strong type="warning">{{ $t('profiles.onlyInA', {name: compareProfileA}) }}
          ({{ diffResult.onlyInA.length }}):
        </n-text>
        <n-tag v-for="mod in diffResult.onlyInA" :key="mod" size="small" style="margin: 2px;" type="warning">{{
            mod
          }}
        </n-tag>
      </div>
      <div v-if="diffResult.onlyInB.length > 0" style="margin-bottom: 8px;">
        <n-text strong type="info">{{ $t('profiles.onlyInB', {name: compareProfileB}) }} ({{
            diffResult.onlyInB.length
          }}):
        </n-text>
        <n-tag v-for="mod in diffResult.onlyInB" :key="mod" size="small" style="margin: 2px;" type="info">{{
            mod
          }}
        </n-tag>
      </div>
    </div>
    <template #action>
      <n-button ghost @click="showCompareDialog = false">{{ $t('profiles.close') }}</n-button>
    </template>
  </n-modal>
</template>

<script lang="ts" setup>
import { open, save } from "@tauri-apps/plugin-dialog";
import { useMessage } from "naive-ui";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
	active_profile,
	apply_mod_list,
	clear_active_profile,
	compare_profiles,
	create_mod_list,
	delete_mod_list,
	export_profile,
	import_profile,
	mod_lists,
	type ProfileDiff,
	rename_profile,
} from "../../invokes";

const message = useMessage();
const { t } = useI18n();

// Create dialog
const showCreateDialog = ref(false);
const newProfileName = ref("");
const isCreating = ref(false);

// Apply dialog
const showApplyDialog = ref(false);
const isApplying = ref(false);

// Delete dialog
const showDeleteDialog = ref(false);
const isDeleting = ref(false);

// Rename dialog
const showRenameDialog = ref(false);
const isRenaming = ref(false);

// Compare dialog
const showCompareDialog = ref(false);
const isComparing = ref(false);
const compareProfileA = ref<string | null>(null);
const compareProfileB = ref<string | null>(null);
const diffResult = ref<ProfileDiff | null>(null);

// Shared target
const targetProfile = ref("");

const profileOptions = computed(() =>
	mod_lists.value
		.filter(
			(p) => p.profileName !== targetProfile.value || showCompareDialog.value,
		)
		.map((p) => ({ label: p.profileName, value: p.profileName })),
);

function clearDiff() {
	diffResult.value = null;
}

async function handleCreate() {
	const name = newProfileName.value.trim();
	if (!name) return;

	isCreating.value = true;
	try {
		const result = await create_mod_list(name);
		message.success(t("profiles.created", { name: result.profileName }));
		showCreateDialog.value = false;
		newProfileName.value = "";
	} catch (error) {
		message.error(String(error));
	} finally {
		isCreating.value = false;
	}
}

function handleApply(name: string) {
	targetProfile.value = name;
	showApplyDialog.value = true;
}

async function confirmApply() {
	isApplying.value = true;
	try {
		await apply_mod_list(targetProfile.value);
		message.success(t("profiles.applied", { name: targetProfile.value }));
		showApplyDialog.value = false;
	} catch (error) {
		message.error(String(error));
	} finally {
		isApplying.value = false;
	}
}

async function handleClearActive() {
	try {
		await clear_active_profile();
		message.success(t("profiles.clearedActive"));
	} catch (error) {
		message.error(String(error));
	}
}

function promptDelete(name: string) {
	targetProfile.value = name;
	showDeleteDialog.value = true;
}

async function confirmDelete() {
	isDeleting.value = true;
	try {
		await delete_mod_list(targetProfile.value);
		message.success(t("profiles.deleted", { name: targetProfile.value }));
		showDeleteDialog.value = false;
	} catch (error) {
		message.error(String(error));
	} finally {
		isDeleting.value = false;
	}
}

function promptRename(name: string) {
	targetProfile.value = name;
	newProfileName.value = name;
	showRenameDialog.value = true;
}

async function confirmRename() {
	const name = newProfileName.value.trim();
	if (!name || name === targetProfile.value) return;

	isRenaming.value = true;
	try {
		await rename_profile(targetProfile.value, name);
		message.success(
			t("profiles.renamed", { old: targetProfile.value, new: name }),
		);
		showRenameDialog.value = false;
		newProfileName.value = "";
	} catch (error) {
		message.error(String(error));
	} finally {
		isRenaming.value = false;
	}
}

function promptCompare(name: string) {
	targetProfile.value = name;
	compareProfileA.value = name;
	compareProfileB.value = null;
	diffResult.value = null;
	showCompareDialog.value = true;
}

async function confirmCompare() {
	if (!compareProfileA.value || !compareProfileB.value) return;

	isComparing.value = true;
	try {
		diffResult.value = await compare_profiles(
			compareProfileA.value,
			compareProfileB.value,
		);
	} catch (error) {
		message.error(String(error));
	} finally {
		isComparing.value = false;
	}
}

async function handleImport() {
	try {
		const selected = await open({
			filters: [{ name: "Profile XML", extensions: ["xml"] }],
			multiple: false,
		});
		if (!selected) return;
		const result = await import_profile(selected as string);
		message.success(t("profiles.imported", { name: result.profileName }));
	} catch (error) {
		message.error(String(error));
	}
}

async function handleExport() {
	try {
		const selected = await save({
			filters: [{ name: "Profile XML", extensions: ["xml"] }],
		});
		if (!selected) return;
		const name = targetProfile.value || mod_lists.value[0]?.profileName;
		if (!name) {
			message.error(t("profiles.noProfiles"));
			return;
		}
		await export_profile(name, selected as string);
		message.success(t("profiles.exported", { name }));
	} catch (error) {
		message.error(String(error));
	}
}
</script>

<style scoped>
.profiles-container {
  padding: 8px;
}

.profiles-header {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 16px;
}

.profiles-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
}

.profile-info {
  display: flex;
  gap: 12px;
  color: var(--n-text-color-2);
  font-size: 13px;
}

.profile-base {
  background: rgba(0, 0, 0, 0.05);
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: 500;
}

:global(.dark) .profile-base {
  background: rgba(255, 255, 255, 0.1);
}
</style>
