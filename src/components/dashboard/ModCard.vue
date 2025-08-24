<template>
  <n-card :segmented="{ content: true }" :title="mod.name" hoverable>
    <!-- Mod Version & Game Version -->
    <n-space justify="space-between" style="margin-bottom: 12px">
      <n-tag v-if="mod.modVersion" size="small" type="info">
        v{{ mod.modVersion }}
      </n-tag>
      <n-tag v-if="mod.gameVersion" :bordered="false" size="small">
        Game: {{ mod.gameVersion }}
      </n-tag>
    </n-space>

    <!-- Core Package Badge -->
    <n-tag
        v-if="mod.corePackage"
        size="small"
        style="margin-bottom: 12px"
        type="success"
    >
      Core Package
    </n-tag>

    <!-- Steam Workshop ID with Link -->
    <n-space v-if="mod.steamWorkshopId" align="center" style="margin-bottom: 12px">
      <n-text depth="3">Steam Workshop ID:</n-text>
      <n-text depth="1" style="font-family: monospace">
        {{ mod.steamWorkshopId }}
      </n-text>
      <n-button
          :href="`https://steamcommunity.com/sharedfiles/filedetails/?id=${mod.steamWorkshopId}`"
          size="tiny"
          tag="a"
          target="_blank"
          text
          type="primary"
      >
        🔗
      </n-button>
    </n-space>

    <!-- Expected Hash -->
    <n-space v-if="mod.expectedHash" align="center" style="margin-bottom: 12px">
      <n-text depth="3">Expected Hash:</n-text>
      <n-code :code="mod.expectedHash" language="txt" size="small"/>
    </n-space>

    <!-- File Groups -->
    <n-collapse v-if="Object.keys(mod.fileGroups).length > 0" style="margin-top: 12px">
      <n-collapse-item :arrow-icon-size="16" title="File Groups">
        <n-list size="small">
          <n-list-item v-for="(group, tag) in mod.fileGroups" :key="tag">
            <template #prefix>
              <n-tag size="small" type="warning">{{ tag }}</n-tag>
            </template>
            <n-text depth="1">{{ group.files.length }} files</n-text>
          </n-list-item>
        </n-list>
      </n-collapse-item>
    </n-collapse>

    <!-- Home Directory -->
    <n-text v-if="mod.homeDir" depth="3" style="font-size: 0.9em; margin-top: 12px">
      Home:
      <n-code :code="mod.homeDir" language="path" size="small"/>
    </n-text>
  </n-card>
</template>

<script lang="ts" setup>
import type {BarotraumaMod} from '../../proto/mods.ts'; // 确保路径正确
defineProps<{
  mod: BarotraumaMod;
}>();
</script>