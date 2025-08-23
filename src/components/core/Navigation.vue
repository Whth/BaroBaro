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
import {computed, h, ref} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import {NIcon} from 'naive-ui'
import {ExtensionPuzzleOutline, HomeOutline, SettingsOutline} from '@vicons/ionicons5'
import {useI18n} from "vue-i18n";

const {t} = useI18n()

// 路由相关
const route = useRoute()
const router = useRouter()

// 折叠状态
const isCollapsed = ref(false)

// 当前激活的菜单项
const activeKey = computed(() => {
  const map: Record<string, string> = {
    '/': 'dashboard',
    '/mods': 'mods',
    '/settings': 'settings'
  }
  return map[route.path] || ''
})

// 菜单项配置
const menuOptions = [
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
]

const goTo = (key: string) => {
  const item = menuOptions.find(opt => opt.key === key)
  if (item?.path) router.push(item.path)
}
</script>

