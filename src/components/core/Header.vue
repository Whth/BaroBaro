<template>
  <header class="app-header">
    <div class="header-content">
      <h1 class="app-title">{{ t('app.title') }}</h1>
      <div class="header-actions">
        <ThemeSwitcher />
        <button class="notification-button" @click="showNotifications">
          <span class="notification-icon">🔔</span>
          <span v-if="unreadNotifications > 0" class="notification-badge">
            {{ unreadNotifications }}
          </span>
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";

import ThemeSwitcher from "./ThemeSwitcher.vue";

const { t } = useI18n();

const unreadNotifications = ref(3);
const showNotifications = () => {
	console.log("Show notifications");
	// Implementation would go here
};
</script>

<style scoped>
.app-header {
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  padding: var(--spacing-s);
  box-shadow: var(--shadow-level-1);
  z-index: 100;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.app-title {
  margin: 0;
  font-size: var(--font-size-heading-2);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-m);
}

.notification-button {
  position: relative;
  background: none;
  border: none;
  cursor: pointer;
  font-size: var(--font-size-body-large);
  color: var(--color-text-primary);
}

.notification-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  background-color: var(--color-error);
  color: white;
  border-radius: 50%;
  font-size: var(--font-size-caption);
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
