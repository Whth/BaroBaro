<template>
  <n-scrollbar style="max-height: 60vh">
    <div class="profiles-container">
      <!-- Save current mods as profile -->
      <div class="profiles-header">
        <n-button type="primary" @click="showCreateDialog = true">
          {{ $t('profiles.saveCurrent') }}
        </n-button>
      </div>

      <!-- Empty state -->
      <n-empty v-if="mod_lists.length === 0" :description="$t('profiles.noProfiles')" style="margin-top: 40px"/>

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
  </n-scrollbar>

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
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { mod_lists, create_mod_list, delete_mod_list, apply_mod_list } from "../../invokes";
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";

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

// Shared target
const targetProfile = ref("");

async function handleCreate() {
  const name = newProfileName.value.trim();
  if (!name) return;

  isCreating.value = true;
  try {
    const result = await create_mod_list(name);
    message.success(t("profiles.created", {name: result.profileName}));
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
    message.success(t("profiles.applied", {name: targetProfile.value}));
    showApplyDialog.value = false;
  } catch (error) {
    message.error(String(error));
  } finally {
    isApplying.value = false;
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
    message.success(t("profiles.deleted", {name: targetProfile.value}));
    showDeleteDialog.value = false;
  } catch (error) {
    message.error(String(error));
  } finally {
    isDeleting.value = false;
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
