<template>
  <n-layout-sider
      :collapsed="isCollapsed"
      :collapsed-width="65"
      :width="180"
      bordered
      collapse-mode="width"
      show-trigger="bar"
      @collapse="handleCollapse"
      @expand="handleExpand"
  >
    <div class="sidebar-header">
      <n-text v-if="!isCollapsed" strong style="font-size: 18px; color: var(--n-text-color-primary);">
        Mod Manager
      </n-text>
    </div>
    <n-menu
        :collapsed="isCollapsed"
        :collapsed-icon-size="26"
        :collapsed-width="64"
        :options="menuOptions"
        :value="activeKey"
        @update:value="handleMenuSelect"
    />
  </n-layout-sider>
</template>

<script lang="ts" setup>
import {computed, h, provide, ref} from "vue";
import {useRoute, useRouter} from "vue-router";
import {useI18n} from "vue-i18n";
import {NIcon} from "naive-ui";
import {
  ExtensionPuzzleOutline as ModsIcon,
  HomeOutline as HomeIcon,
  SettingsOutline as SettingsIcon,
} from "@vicons/ionicons5";

const route = useRoute();
const router = useRouter();
const {t} = useI18n();

const isCollapsed = ref(false);

const activeKey = computed(() => {
  if (route.path === "/") return "dashboard";
  if (route.path === "/mods") return "mods";
  if (route.path === "/settings") return "settings";
  return null;
});

const menuOptions = computed(() => [
  {
    label: t("navigation.dashboard"),
    key: "dashboard",
    path: "/",
    icon: () => h(NIcon, null, {default: () => h(HomeIcon)}),
  },
  {
    label: t("navigation.mods"),
    key: "mods",
    path: "/mods",
    icon: () => h(NIcon, null, {default: () => h(ModsIcon)}),
  },
  {
    label: t("navigation.settings"),
    key: "settings",
    path: "/settings",
    icon: () => h(NIcon, null, {default: () => h(SettingsIcon)}),
  },
]);

const handleMenuSelect = (key: string) => {
  const option = menuOptions.value.find((opt) => opt.key === key);
  if (option && option.path) {
    router.push(option.path);
  }
};

const handleCollapse = () => {
  isCollapsed.value = true;
};

const handleExpand = () => {
  isCollapsed.value = false;
};

// Provide sidebar state to parent components
provide("sidebarState", {
  isCollapsed: isCollapsed,
  toggleSidebar: () => {
    isCollapsed.value = !isCollapsed.value;
  },
});
</script>

<style scoped>
.sidebar-header {
  padding: 16px;
  border-bottom: 1px solid var(--n-border-color);
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 64px;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-bottom: 1px solid rgba(255, 255, 255, 0.15);
}

[data-theme="dark"] .sidebar-header {
  border-color: rgba(255, 255, 255, 0.08);
  border-bottom-color: rgba(255, 255, 255, 0.1);
}

:deep(.n-layout-sider) {
  background: rgba(255, 255, 255, 0.1);
  border-right: 1px solid rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

[data-theme="dark"] :deep(.n-layout-sider) {
  background: rgba(31, 41, 55, 0.4);
  border-right-color: rgba(255, 255, 255, 0.1);
}

:deep(.n-menu) {
  background: var(--n-color-menu-color);
  color: var(--n-color-menu-text-color);
}

:deep(.n-menu-item) {
  color: var(--n-color-menu-item-text-color);
}

:deep(.n-menu-item:hover) {
  background: var(--n-color-menu-item-color-hover);
  color: var(--n-color-menu-item-text-color-hover);
}

:deep(.n-menu-item--active) {
  background: var(--n-color-menu-item-color-active);
  color: var(--n-color-menu-item-text-color-active);
}
</style>

