<template>
  <n-layout-sider
    :collapsed="isCollapsed"
    :collapsed-width="64"
    :width="280"
    bordered
    show-trigger="bar"
    collapse-mode="width"
    @collapse="handleCollapse"
    @expand="handleExpand"
  >
    <n-menu
      :collapsed="isCollapsed"
      :collapsed-width="64"
      :collapsed-icon-size="24"
      :options="menuOptions"
      :value="activeKey"
      @update:value="handleMenuSelect"
    />
  </n-layout-sider>
</template>

<script setup lang="ts">
import { ref, provide, computed, h } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { NIcon } from "naive-ui";
import {
	HomeOutline as HomeIcon,
	SettingsOutline as SettingsIcon,
	ExtensionPuzzleOutline as ModsIcon,
} from "@vicons/ionicons5";

const route = useRoute();
const router = useRouter();
const { t } = useI18n();

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
		icon: () => h(NIcon, null, { default: () => h(HomeIcon) }),
	},
	{
		label: t("navigation.mods"),
		key: "mods",
		path: "/mods",
		icon: () => h(NIcon, null, { default: () => h(ModsIcon) }),
	},
	{
		label: t("navigation.settings"),
		key: "settings",
		path: "/settings",
		icon: () => h(NIcon, null, { default: () => h(SettingsIcon) }),
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
:deep(.n-layout-sider) {
  height: 100vh;
}

:deep(.n-menu) {
  height: 100%;
}

:deep(.n-menu-item) {
  margin-bottom: 8px;
}

:deep(.n-menu-item-content) {
  padding: 12px 16px;
  border-radius: 8px;
  margin: 4px 8px;
  transition: all 0.3s ease;
}

:deep(.n-menu-item-content:hover) {
  background-color: rgba(255, 255, 255, 0.1);
  transform: translateX(4px);
}

:deep(.n-menu-item--selected .n-menu-item-content) {
  background-color: #2080f0;
  color: white;
  box-shadow: 0 2px 8px rgba(32, 128, 240, 0.3);
}
</style>
