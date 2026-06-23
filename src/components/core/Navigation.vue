<template>
  <n-layout-sider
      v-model:collapsed="isCollapsed"
      bordered
      collapse-mode="width"
      show-trigger="bar"
      width="200"
      class="nav-sidebar"
      :native-scrollbar="false"
  >
    <div class="nav-brand">
      <n-text v-if="!isCollapsed" strong class="nav-brand-text">BaroBaro</n-text>
      <n-icon v-else size="24" class="nav-brand-icon"><HomeOutline /></n-icon>
    </div>
    <n-menu
        v-model:value="activeKey"
        :collapsed="isCollapsed"
        :options="menuOptions"
        @update:value="goTo"
    />
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
		const item = menuOptions.value.find((opt) => opt.key === key);
		if (item?.path) router.push(item.path);
	},
});

// Menu item configuration
const menuOptions = computed(() => [
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
	{
		label: t("navigation.settings"),
		key: "settings",
		path: "/settings",
		icon: () => h(NIcon, null, { default: () => h(SettingsOutline) }),
	},
]);

const goTo = (key: string) => {
	const item = menuOptions.value.find((opt) => opt.key === key);
	if (item?.path) router.push(item.path);
};
</script>

<style scoped>

.nav-brand {
  padding: 20px 16px;
  text-align: center;
  border-bottom: 1px solid var(--color-border);
  margin-bottom: 8px;
}

.nav-brand-text {
  font-size: var(--text-lg);
  font-weight: 700;
  background: linear-gradient(135deg, var(--color-primary), var(--color-info));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
.nav-brand-icon {
  color: var(--color-primary);
  transition: color var(--transition-base);
}
</style>
