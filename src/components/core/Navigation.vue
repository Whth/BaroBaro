<template>
  <n-layout-sider
      :collapsed="isCollapsed"
      :collapsed-width="80"
      :width="280"
      bordered
      collapse-mode="width"
      show-trigger="bar"
      @collapse="handleCollapse"
      @expand="handleExpand"
  >
    <n-menu
        :collapsed="isCollapsed"
        :collapsed-icon-size="24"
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

