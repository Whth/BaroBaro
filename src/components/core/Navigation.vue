<template>
  <nav class="app-navigation" :class="{ 'collapsed': isCollapsed }">
    <ul class="nav-list">
      <li class="nav-item toggle-item">
        <button class="nav-link toggle-btn" @click="toggleSidebar" :title="isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'">
          <span class="nav-icon">{{ isCollapsed ? '→' : '←' }}</span>
          <span class="nav-text">{{ isCollapsed ? t('navigation.expand') : t('navigation.collapse') }}</span>
        </button>
      </li>
      <li class="nav-item">
        <router-link to="/" class="nav-link" :class="{ active: isActive('/') }" :title="isCollapsed ? t('navigation.dashboard') : ''">
          <span class="nav-icon">📊</span>
          <span class="nav-text">{{ t('navigation.dashboard') }}</span>
        </router-link>
      </li>
      <li class="nav-item">
        <router-link
          to="/mods"
          class="nav-link"
          :class="{ active: isActive('/mods') }"
          :title="isCollapsed ? t('navigation.mods') : ''"
        >
          <span class="nav-icon">🧩</span>
          <span class="nav-text">{{ t('navigation.mods') }}</span>
        </router-link>
      </li>
      <li class="nav-item">
        <router-link
          to="/settings"
          class="nav-link"
          :class="{ active: isActive('/settings') }"
          :title="isCollapsed ? t('navigation.settings') : ''"
        >
          <span class="nav-icon">⚙️</span>
          <span class="nav-text">{{ t('navigation.settings') }}</span>
        </router-link>
       </li>
     </ul>
  </nav>
</template>

<script setup lang="ts">
import { ref, provide } from "vue";
import { useRoute } from "vue-router";
import { useI18n } from "vue-i18n";

const route = useRoute();
const { t } = useI18n();

const isCollapsed = ref(false);

const isActive = (path: string) => {
	return route.path === path;
};

const toggleSidebar = () => {
	isCollapsed.value = !isCollapsed.value;
};

// Provide sidebar state to parent components
provide('sidebarState', {
	isCollapsed: isCollapsed,
	toggleSidebar: toggleSidebar
});
</script>

<style scoped>
.app-navigation {
  width: 250px;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  padding: var(--spacing-xl) var(--spacing-m) var(--spacing-m);
  display: flex;
  flex-direction: column;
  height: 100vh;
  position: sticky;
  top: 0;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
}

.app-navigation.collapsed {
  width: 60px;
  padding: var(--spacing-xl) var(--spacing-xs) var(--spacing-m);
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
  flex: 1;
}

.nav-item {
	margin-bottom: var(--spacing-m);
	opacity: 0;
	transform: translateX(-30px);
	animation: slideInLeft 0.6s cubic-bezier(0.4, 0, 0.2, 1) forwards;
}

.app-navigation.collapsed .nav-item {
	margin-bottom: var(--spacing-s);
}

.nav-item:nth-child(1) { animation-delay: 0.1s; }
.nav-item:nth-child(2) { animation-delay: 0.2s; }
.nav-item:nth-child(3) { animation-delay: 0.3s; }
.nav-item:nth-child(4) { animation-delay: 0.4s; }

@keyframes slideInLeft {
	from {
		opacity: 0;
		transform: translateX(-20px);
	}
	to {
		opacity: 1;
		transform: translateX(0);
	}
}

.nav-link {
	display: flex;
	align-items: center;
	padding: var(--spacing-s);
	border-radius: var(--border-radius-soft);
	text-decoration: none;
	color: var(--color-text-primary);
	transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	position: relative;
	overflow: hidden;
	justify-content: flex-start;
	transform: translateX(0);
}

.nav-link::after {
	content: '';
	position: absolute;
	left: 0;
	top: 0;
	height: 100%;
	width: 0;
	background: linear-gradient(90deg, var(--color-primary), transparent);
	transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.nav-link:hover::after {
	width: 4px;
}

.nav-link:hover {
  background-color: var(--color-background);
  transform: translateX(8px);
}

.nav-link.active {
  background-color: var(--color-primary);
  color: white;
  animation: activePulse 2s infinite;
}

.nav-link.active::after {
	width: 4px;
}

@keyframes activePulse {
  0%, 100% {
    box-shadow: 0 0 0 0 rgba(var(--color-primary-rgb), 0.4);
  }
  50% {
    box-shadow: 0 0 0 8px rgba(var(--color-primary-rgb), 0);
  }
}

.nav-icon {
  margin-right: var(--spacing-s);
  font-size: var(--font-size-body-large);
}

.nav-text {
  font-weight: var(--font-weight-medium);
  transition: opacity 0.3s ease, transform 0.3s ease;
  display: inline;
}

.app-navigation.collapsed .nav-text {
  opacity: 0;
  transform: translateX(-10px);
  pointer-events: none;
  display: none;
}

.app-navigation.collapsed .nav-link {
  justify-content: center;
  padding: var(--spacing-s) var(--spacing-xs);
  min-height: 44px;
  flex-direction: column;
  align-items: center;
}

.app-navigation.collapsed .nav-icon {
  margin-right: 0;
  margin-bottom: 0;
}

.app-navigation.collapsed .nav-icon {
  font-size: var(--font-size-body-large);
}

.app-navigation.collapsed .nav-link:hover {
  transform: scale(1.1);
}

/* Toggle button styles */
.toggle-item {
  border-bottom: 1px solid var(--color-border);
  margin-bottom: var(--spacing-m);
  padding-bottom: var(--spacing-s);
}

.toggle-btn {
  width: 100%;
  background: none;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: var(--font-size-body);
}

.toggle-btn:hover {
  background-color: var(--color-background);
  color: var(--color-text-primary);
}

.app-navigation.collapsed .toggle-btn {
  justify-content: center;
  flex-direction: column;
  align-items: center;
}

.app-navigation.collapsed .toggle-btn .nav-text {
  display: none;
}

</style>
