<template>
  <nav class="app-navigation">
    <ul class="nav-list">
      <li class="nav-item">
        <router-link to="/" class="nav-link" :class="{ active: isActive('/') }">
          <span class="nav-icon">📊</span>
          <span class="nav-text">{{ t('navigation.dashboard') }}</span>
        </router-link>
     </li>
     <li class="nav-item">
       <router-link
         to="/mods"
         class="nav-link"
         :class="{ active: isActive('/mods') }"
       >
         <span class="nav-icon">🧩</span>
         <span class="nav-text">{{ t('navigation.mods') }}</span>
       </router-link>
     </li>
     <li class="nav-item">
       <router-link
         to="/profiles"
         class="nav-link"
         :class="{ active: isActive('/profiles') }"
       >
         <span class="nav-icon">👤</span>
         <span class="nav-text">{{ t('navigation.profiles') }}</span>
       </router-link>
     </li>
     <li class="nav-item">
       <router-link
         to="/settings"
         class="nav-link"
         :class="{ active: isActive('/settings') }"
       >
         <span class="nav-icon">⚙️</span>
         <span class="nav-text">{{ t('navigation.settings') }}</span>
       </router-link>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import { useI18n } from "vue-i18n";

const route = useRoute();
const { t } = useI18n();

const isActive = (path: string) => {
	return route.path === path;
};
</script>

<style scoped>
.app-navigation {
  width: 250px;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  padding: var(--spacing-m);
  height: calc(100vh - 120px);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
	margin-bottom: var(--spacing-s);
	opacity: 0;
	transform: translateX(-20px);
	animation: slideInLeft 0.6s ease-out forwards;
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
}

.nav-link::before {
	content: '';
	position: absolute;
	left: 0;
	top: 0;
	height: 100%;
	width: 0;
	background: linear-gradient(90deg, var(--color-primary), transparent);
	transition: width 0.3s ease;
}

.nav-link:hover::before {
	width: 4px;
}

.nav-link:hover {
  background-color: var(--color-background);
}

.nav-link.active {
  background-color: var(--color-primary);
  color: white;
}

.nav-icon {
  margin-right: var(--spacing-s);
  font-size: var(--font-size-body-large);
}

.nav-text {
  font-weight: var(--font-weight-medium);
}
</style>
