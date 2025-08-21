<template>
  <nav class="app-navigation">
    <ul class="nav-list">
      <li class="nav-item">
        <router-link to="/" class="nav-link" :class="{ active: isActive('/') }">
          <span class="nav-icon">📊</span>
          <span class="nav-text">Dashboard</span>
        </router-link>
      </li>
      <li class="nav-item">
        <router-link to="/mods" class="nav-link" :class="{ active: isActive('/mods') }">
          <span class="nav-icon">🧩</span>
          <span class="nav-text">Mods</span>
        </router-link>
      </li>
      <li class="nav-item">
        <router-link to="/profiles" class="nav-link" :class="{ active: isActive('/profiles') }">
          <span class="nav-icon">👤</span>
          <span class="nav-text">Profiles</span>
        </router-link>
      </li>
      <li class="nav-item">
        <router-link to="/settings" class="nav-link" :class="{ active: isActive('/settings') }">
          <span class="nav-icon">⚙️</span>
          <span class="nav-text">Settings</span>
        </router-link>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const isActive = (path: string) => {
  return route.path === path
}
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
}

.nav-link {
  display: flex;
  align-items: center;
  padding: var(--spacing-s);
  border-radius: var(--border-radius-soft);
  text-decoration: none;
  color: var(--color-text-primary);
  transition: background-color 0.2s;
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