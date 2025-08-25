<template>
  <n-card hoverable>
    <n-thing>
      <template #header>
        <div style="display: flex; align-items: center; gap: 12px; flex-wrap: wrap;">

          <n-text strong style="font-size: 16px; ">{{ `${index}. ` }}</n-text>
          <n-text strong style="font-size: 16px;">{{ mod.name }}</n-text>
          <n-tag v-if="mod.corePackage" size="small" type="info">Core Package</n-tag>
          <n-tag v-if="mod.homeDir" size="small" type="success">Local Mod</n-tag>
        </div>
      </template>

      <template #description>
        <n-space :size="6" style="margin-top: 8px;" vertical>
          <n-descriptions :column="4" size="small">
            <!-- Mod Version -->
            <n-descriptions-item :label="$t('modCard.version')">
              <n-text code>{{ mod.modVersion }}</n-text>
            </n-descriptions-item>

            <!-- Game Version -->
            <n-descriptions-item :label="$t('modCard.gameVersion')">
              <n-text code>{{ mod.gameVersion }}</n-text>
            </n-descriptions-item>
            <!-- Expected Hash -->
            <n-descriptions-item :label="$t('modCard.expectedHash')">
              <n-text code style="word-break: break-all;">{{ mod.expectedHash.slice(0, 7) }}</n-text>
              <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
                <n-icon size="1.2em" @click="toClipboard(mod.expectedHash)">
                  <copy-outline/>
                </n-icon>
              </n-button>
            </n-descriptions-item>

            <!-- Steam Workshop ID -->
            <n-descriptions-item :label="$t('modCard.steamWorkshopId')">
              <n-text code>{{ mod.steamWorkshopId }}</n-text>

              <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
                <n-icon size="1.2em" @click="toClipboard(mod.steamWorkshopId)">
                  <copy-outline/>
                </n-icon>
              </n-button>

              <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
                <n-icon size="1.2em"
                        @click="openUrl(`https://steamcommunity.com/sharedfiles/filedetails/?id=${mod.steamWorkshopId}`)">
                  <open-outline/>
                </n-icon>
              </n-button>
            </n-descriptions-item>


            <!-- Home Directory -->
            <n-descriptions-item v-if="mod.homeDir" :label="$t('modCard.homeDir')">
              <n-text code style="word-break: break-all;">{{ mod.homeDir }}</n-text>

              <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
                <n-icon size="1.2em" @click="toClipboard(mod.homeDir)">
                  <copy-outline/>
                </n-icon>
              </n-button>

              <n-button size="small" style="margin-left: 0.5em; vertical-align: text-bottom" text>
                <n-icon size="1.2em" @click="revealItemInDir(mod.homeDir)">
                  <open-outline/>
                </n-icon>
              </n-button>
            </n-descriptions-item>
          </n-descriptions>
        </n-space>
      </template>
    </n-thing>
  </n-card>
</template>

<script lang="ts" setup>
import type {BarotraumaMod} from '../../proto/mods.ts'
import {openUrl, revealItemInDir} from '@tauri-apps/plugin-opener'
import {CopyOutline, OpenOutline} from '@vicons/ionicons5'
import useClipboard from 'vue-clipboard3'

const {toClipboard} = useClipboard()

interface Props {
  mod: BarotraumaMod,
  index: number
}

defineProps<Props>()
</script>