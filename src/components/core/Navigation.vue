<template>
  <n-layout-sider
      v-model:collapsed="isCollapsed"
      :native-scrollbar="false"
      bordered
      class="nav-sidebar"
      collapse-mode="width"
      show-trigger="bar"
      width="200"
  >
    <div class="nav-menu-wrap">
      <n-menu
          v-model:value="activeKey"
          :collapsed="isCollapsed"
          :options="mainMenuOptions"
          @update:value="goTo"
      />
      <n-menu
          v-model:value="activeKey"
          :collapsed="isCollapsed"
          :options="bottomMenuOptions"
          @update:value="goTo"
      />
    </div>
  </n-layout-sider>
</template>

<script lang="ts" setup>
import {
	ExtensionPuzzleOutline,
	HomeOutline,
	SettingsOutline,
} from "@vicons/ionicons5";
import { NIcon } from "naive-ui";
import { computed, h, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRoute, useRouter } from "vue-router";

const { t } = useI18n();

// Router related
const route = useRoute();
const router = useRouter();

// Collapsed state - 从 localStorage 读取初始值
const isCollapsed = ref(localStorage.getItem("sidebarCollapsed") === "true");

// 监听 isCollapsed 变化并保存到 localStorage
watch(isCollapsed, (newVal) => {
	localStorage.setItem("sidebarCollapsed", String(newVal));
});

// Currently active menu item
const activeKey = computed({
	get: () => {
		const map: Record<string, string> = {
			"/": "dashboard",
			"/mods": "mods",
			"/settings": "settings",
		};
		return map[route.path] || "";
	},
	set: (key) => {
		const item = [...mainMenuOptions.value, ...bottomMenuOptions.value].find(
			(opt) => opt.key === key,
		);
		if (item?.path) router.push(item.path);
	},
});

// Top menu items
const mainMenuOptions = computed(() => [
	{
		label: t("navigation.dashboard"),
		key: "dashboard",
		path: "/",
		icon: () => h(NIcon, null, { default: () => h(HomeOutline) }),
	},
	{
		label: t("navigation.mods"),
		key: "mods",
		path: "/mods",
		icon: () => h(NIcon, null, { default: () => h(ExtensionPuzzleOutline) }),
	},
]);

// Bottom-pinned menu items
const bottomMenuOptions = computed(() => [
	{
		label: t("navigation.settings"),
		key: "settings",
		path: "/settings",
		icon: () => h(NIcon, null, { default: () => h(SettingsOutline) }),
	},
]);

const goTo = (key: string) => {
	const item = [...mainMenuOptions.value, ...bottomMenuOptions.value].find(
		(opt) => opt.key === key,
	);
	if (item?.path) router.push(item.path);
};
</script>

<style scoped>
.nav-sidebar :deep(.n-layout-sider-scroll-container) {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.nav-menu-wrap {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
}

.nav-menu-wrap > .n-menu:first-child {
  flex: 1;
}

.nav-menu-wrap > .n-menu:last-child {
  margin-top: auto;
  border-top: 1px solid var(--n-border-color);
}
</style>
