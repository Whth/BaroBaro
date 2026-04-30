<template>
  <n-scrollbar style="max-height: 60vh">
    <n-empty v-if="mods.length === 0" :description="$t('modManagement.noInstalledMods')"/>

    <div v-else>
      <!-- Bulk actions -->
      <div class="mg-bulk-actions">
        <slot name="bulk-actions"/>
        <n-button
            :disabled="selectedMods.size === 0"
            class="mg-action-btn"
            ghost
            size="medium"
            @click="$emit('clear-selection')"
        >
          {{ $t('modManagement.clearSelection') }}
        </n-button>
      </div>

      <!-- Mod grid -->
      <div class="mg-grid">
        <div
            v-for="mod in mods"
            :key="mod.steamWorkshopId"
            :class="{ selected: selectedMods.has(mod.steamWorkshopId) }"
            class="mg-item"
            @click="$emit('toggle-select', mod.steamWorkshopId)"
        >
          <div class="mg-content">
            <div class="mg-info">
              <div class="mg-header">
                <h4 class="mg-title">{{ mod.name }}</h4>
                <div class="mg-tags">
                  <n-tag
                      v-for="tag in mod.tags.slice(0, 3)"
                      :key="tag"
                      :style="getTagStyle(tag)"
                      round
                      size="small"
                  >
                    {{ tag }}
                  </n-tag>
                  <span v-if="mod.tags.length > 3" class="mg-more-tags">+{{ mod.tags.length - 3 }}</span>
                </div>
              </div>
              <div class="mg-details">
                <span class="mg-detail-item">v{{ mod.modVersion }}</span>
                <span class="mg-detail-item">Game: {{ mod.gameVersion }}</span>
              </div>
            </div>
            <slot name="mod-actions" :mod="mod"/>
          </div>
        </div>
      </div>
    </div>
  </n-scrollbar>
</template>

<script lang="ts" setup>
import type { BarotraumaMod } from "../../proto/mods.ts";
import getTagColorConfig from "../../composables/coloredTag.ts";

defineProps<{
  mods: BarotraumaMod[];
  selectedMods: Set<number>;
}>();

defineEmits<{
  "toggle-select": [modId: number];
  "clear-selection": [];
}>();

function getTagStyle(tag: string) {
  const config = getTagColorConfig(tag);
  return {
    cursor: "pointer",
    transition: "all 0.2s ease",
    opacity: "0.85",
    ...config,
  };
}
</script>

<style scoped>
/* Bulk actions */
.mg-bulk-actions {
  display: flex;
  gap: 8px;
  justify-content: center;
  margin-bottom: 12px;
  padding: 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  backdrop-filter: blur(10px);
}

.mg-action-btn {
  transition: all 0.15s ease;
}

.mg-action-btn:hover:not(:disabled) {
  transform: translateY(-1px);
}

/* Mod grid */
.mg-grid {
  display: grid;
  gap: 8px;
  grid-template-columns: 1fr;
}

.mg-item {
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 2px solid transparent;
  background: rgba(255, 255, 255, 0.03);
}

.mg-item:hover {
  background: rgba(255, 255, 255, 0.08);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.mg-item.selected {
  background: rgba(64, 158, 255, 0.1);
  border-color: #409eff;
  box-shadow: 0 0 12px rgba(64, 158, 255, 0.2);
}

/* Content layout */
.mg-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.mg-info {
  flex: 1;
  min-width: 0;
}

.mg-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 8px;
  gap: 12px;
}

.mg-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--n-text-color);
  line-height: 1.2;
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.mg-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  max-width: 200px;
  align-items: center;
}

.mg-more-tags {
  color: var(--n-text-color-2);
  font-size: 11px;
  font-weight: 500;
}

.mg-details {
  display: flex;
  gap: 16px;
  color: var(--n-text-color-2);
  font-size: 12px;
}

.mg-detail-item {
  background: rgba(0, 0, 0, 0.05);
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: 500;
}

/* Responsive */
@media (max-width: 768px) {
  .mg-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .mg-tags {
    max-width: none;
  }

  .mg-details {
    flex-wrap: wrap;
  }
}

/* Dark theme */
:global(.dark) .mg-title {
  color: var(--n-text-color);
}

:global(.dark) .mg-item {
  background: rgba(255, 255, 255, 0.05);
}

:global(.dark) .mg-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

:global(.dark) .mg-detail-item {
  background: rgba(255, 255, 255, 0.1);
  color: var(--n-text-color-2);
}

:global(.dark) .mg-details {
  color: var(--n-text-color-2);
}

:global(.dark) .mg-bulk-actions {
  background: rgba(255, 255, 255, 0.05);
}
</style>
