<template>
  <n-card :bordered="false" class="version-info-card">
    <n-h2>Version Information</n-h2>

    <n-grid :cols="3" :x-gap="16" :y-gap="16">
      <n-gi>
        <n-card hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <GameController/>
              </n-icon>
              <span>Game Mod Manager</span>
            </n-space>
          </template>
          <n-descriptions :column="1" label-placement="left" size="small">
            <n-descriptions-item label="Version">0.1.0</n-descriptions-item>
            <n-descriptions-item label="Build">Alpha</n-descriptions-item>
            <n-descriptions-item label="Release Date">2025</n-descriptions-item>
          </n-descriptions>
        </n-card>
      </n-gi>

      <n-gi>
        <n-card hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <InformationCircle/>
              </n-icon>
              <span>About</span>
            </n-space>
          </template>
          <n-space vertical>
            <n-text>
              A modern, user-friendly mod manager for Barotrauma game.
            </n-text>
            <n-text>
              Built with Vue.js and Tauri for cross-platform compatibility.
            </n-text>
          </n-space>
        </n-card>
      </n-gi>

      <n-gi>
        <n-card hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <Link/>
              </n-icon>
              <span>Links & Support</span>
            </n-space>
          </template>
          <n-space vertical>
            <n-button href="#" tag="a" text type="primary">
              Documentation
            </n-button>
            <n-button href="#" tag="a" text type="primary">
              Support
            </n-button>
            <n-button href="#" tag="a" text type="primary">
              Report Issues
            </n-button>
          </n-space>
        </n-card>
      </n-gi>

      <n-gi>
        <n-card hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <DocumentText/>
              </n-icon>
              <span>Copyright</span>
            </n-space>
          </template>
          <n-space vertical>
            <n-text>© 2025 Game Mod Manager. All rights reserved.</n-text>
            <n-text depth="3">This software is provided as-is without warranty.</n-text>
          </n-space>
        </n-card>
      </n-gi>
    </n-grid>
  </n-card>
</template>

<script lang="ts" setup>
import {DocumentText, GameController, InformationCircle, Link} from '@vicons/ionicons5'
</script>

<style scoped>
.version-info-card {
  height: 100%;
}

:deep(.n-card-header__main) {
  font-weight: 600;
}

/* 响应式：小屏幕时变为2列 */
:deep(.n-grid) {
  --n-grid-cols: 3;
}

@media (max-width: 1200px) {
  :deep(.n-grid) {
    --n-grid-cols: 2;
  }
}

@media (max-width: 768px) {
  :deep(.n-grid) {
    --n-grid-cols: 1;
  }
}
</style>