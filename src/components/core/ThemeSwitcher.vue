<template>
  <div class="theme-switcher">
    <button class="theme-button" @click="toggleThemeMenu">
      <span class="theme-icon">🎨</span>
    </button>
    <div v-if="showThemeMenu" class="theme-menu" ref="themeMenu">
      <div class="theme-menu-header">
        <h3>Theme</h3>
        <button class="close-button" @click="toggleThemeMenu">×</button>
      </div>
      <div class="theme-options">
        <button
          v-for="theme in themes"
          :key="theme.value"
          class="theme-option"
          :class="{ active: currentTheme === theme.value }"
          @click="selectTheme(theme.value)"
        >
          <span class="theme-preview" :class="`theme-${theme.value}`"></span>
          <span class="theme-name">{{ theme.name }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";

interface Theme {
	name: string;
	value: string;
}

const themes: Theme[] = [
	{ name: "Light", value: "light" },
	{ name: "Dark", value: "dark" },
	{ name: "Barotrauma", value: "barotrauma" },
];

const currentTheme = ref("light");
const showThemeMenu = ref(false);
const themeMenu = ref<HTMLElement | null>(null);

const toggleThemeMenu = () => {
	showThemeMenu.value = !showThemeMenu.value;
};

const selectTheme = (theme: string) => {
	currentTheme.value = theme;
	document.documentElement.setAttribute("data-theme", theme);
	localStorage.setItem("theme", theme);
	showThemeMenu.value = false;
};

const handleClickOutside = (event: MouseEvent) => {
	if (themeMenu.value && !themeMenu.value.contains(event.target as Node)) {
		showThemeMenu.value = false;
	}
};

onMounted(() => {
	// Load theme from localStorage or default to 'light'
	const savedTheme = localStorage.getItem("theme") || "light";
	currentTheme.value = savedTheme;

	// Note: Theme is now applied globally in App.vue
	// We just need to sync with the current state

	// Add event listener for clicking outside
	document.addEventListener("click", handleClickOutside);
});

onBeforeUnmount(() => {
	// Clean up event listener
	document.removeEventListener("click", handleClickOutside);
});
</script>

<style scoped>
.theme-switcher {
  position: relative;
}

.theme-button {
  background: none;
  border: none;
  cursor: pointer;
  font-size: var(--font-size-body-large);
  color: var(--color-text-primary);
  padding: var(--spacing-xs);
  border-radius: var(--border-radius-soft);
}

.theme-button:hover {
  background-color: var(--color-background);
}

.theme-menu {
  position: absolute;
  top: 100%;
  right: 0;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  box-shadow: var(--shadow-level-2);
  width: 200px;
  z-index: 1000;
  margin-top: var(--spacing-xs);
}

.theme-menu-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-s) var(--spacing-m);
  border-bottom: 1px solid var(--color-border);
}

.theme-menu-header h3 {
  margin: 0;
  font-size: var(--font-size-body-large);
  color: var(--color-text-primary);
}

.close-button {
  background: none;
  border: none;
  font-size: var(--font-size-heading-3);
  cursor: pointer;
  color: var(--color-text-secondary);
}

.theme-options {
  padding: var(--spacing-s);
}

.theme-option {
  display: flex;
  align-items: center;
  width: 100%;
  padding: var(--spacing-s);
  border: none;
  background: none;
  cursor: pointer;
  border-radius: var(--border-radius-soft);
  text-align: left;
}

.theme-option:hover {
  background-color: var(--color-background);
}

.theme-option.active {
  background-color: var(--color-primary-light);
}

.theme-preview {
  display: inline-block;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  margin-right: var(--spacing-s);
  border: 1px solid var(--color-border);
}

.theme-light {
  background-color: var(--color-background-light);
}

.theme-dark {
  background-color: var(--color-background-dark);
}

.theme-barotrauma {
  background-color: var(--color-background-barotrauma);
}

.theme-name {
  color: var(--color-text-primary);
}
</style>
