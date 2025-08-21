<template>
  <n-dropdown
    :options="themeOptions"
    :value="currentTheme"
    @select="handleThemeSelect"
    trigger="click"
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

const currentTheme = computed(() => themeMode.value);

const themeIcon = computed(() =>
  currentTheme.value === 'dark' ? MoonOutline : SunnyOutline
);

const themeOptions = [
  {
    type: 'default',
    label: "Light",
    key: "light",
  },
  {
    type: 'default',
    label: "Dark",
    key: "dark",
  },
];

const handleThemeSelect = (key: string) => {
  setTheme(key as "light" | "dark");
};
</script>

