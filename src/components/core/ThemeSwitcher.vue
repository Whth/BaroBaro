<template>
  <n-dropdown
    :options="themeOptions"
    :value="themeMode"
    @select="handleThemeSelect"
    trigger="click"
    placement="bottom-end"
  >
    <n-button quaternary circle>
      <template #icon>
        <n-icon :component="themeIcon" />
      </template>
    </n-button>
  </n-dropdown>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { SunnyOutline, MoonOutline } from "@vicons/ionicons5";
import { useTheme } from "../../composables/useTheme";
import { NIcon, NButton, NDropdown } from "naive-ui";

const { themeMode, setTheme } = useTheme();

const themeIcon = computed(() =>
	themeMode.value === "dark" ? MoonOutline : SunnyOutline,
);

const themeOptions = [
	{
		label: "Light",
		key: "light",
	},
	{
		label: "Dark",
		key: "dark",
	},
];

const handleThemeSelect = (key: string) => {
	setTheme(key as "light" | "dark");
};
</script>

