<template>
  <n-layout-sider
      v-model:collapsed="isCollapsed"
      :native-scrollbar="false"
      bordered
      collapse-mode="width"
      show-trigger="bar"
  >
    <n-menu
        v-model:value="activeKey"
        :collapsed="isCollapsed"
        :options="menuOptions"
        @update:value="goTo"
    />
  </n-layout-sider>
</template>

<script lang="ts" setup>
import {computed, h, ref, watch} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import {NIcon} from 'naive-ui'
import {ExtensionPuzzleOutline, HomeOutline, SettingsOutline} from '@vicons/ionicons5'
import {useI18n} from "vue-i18n";

const {t} = useI18n()

// Router related
const route = useRoute()
const router = useRouter()

// Collapsed state - 从 localStorage 读取初始值
const isCollapsed = ref(localStorage.getItem('sidebarCollapsed') === 'true')

// 监听 isCollapsed 变化并保存到 localStorage
watch(isCollapsed, (newVal) => {
  localStorage.setItem('sidebarCollapsed', String(newVal))
})

// Currently active menu item
const activeKey = computed(() => {
  const map: Record<string, string> = {
    '/': 'dashboard',
    '/mods': 'mods',
    '/settings': 'settings'
  }
  return map[route.path] || ''
})

// Menu item configuration
const menuOptions = computed(() => [
  {
    label: t('navigation.dashboard'),
    key: 'dashboard',
    path: '/',
    icon: () => h(NIcon, null, {default: () => h(HomeOutline)})
  },
  {
    label: t('navigation.mods'),
    key: 'mods',
    path: '/mods',
    icon: () => h(NIcon, null, {default: () => h(ExtensionPuzzleOutline)})
  },
  {
    label: t('navigation.settings'),
    key: 'settings',
    path: '/settings',
    icon: () => h(NIcon, null, {default: () => h(SettingsOutline)})
  }
])

const goTo = (key: string) => {
  const item = menuOptions.value.find(opt => opt.key === key)
  if (item?.path) router.push(item.path)
}
</script>