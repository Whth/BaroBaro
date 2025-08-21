<template>
  <div class="language-switcher">
    <select
      v-model="selectedLanguage"
      @change="changeLanguage"
      class="language-select"
    >
      <option value="en">{{ t('settings.english') }}</option>
      <option value="zh">{{ t('settings.chinese') }}</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { locale, t } = useI18n()
const selectedLanguage = ref(locale.value)

const changeLanguage = (event: Event) => {
  const target = event.target as HTMLSelectElement
  const newLocale = target.value
  locale.value = newLocale
  selectedLanguage.value = newLocale
  localStorage.setItem('userLocale', newLocale)
}

onMounted(() => {
  // Load saved language preference
  const savedLocale = localStorage.getItem('userLocale')
  if (savedLocale && (savedLocale === 'en' || savedLocale === 'zh')) {
    locale.value = savedLocale
    selectedLanguage.value = savedLocale
  }
})
</script>

<style scoped>
.language-switcher {
  display: flex;
  align-items: center;
}

.language-select {
  padding: var(--spacing-xs) var(--spacing-s);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
  cursor: pointer;
  transition: all 0.2s ease;
}

.language-select:hover {
  border-color: var(--color-primary);
}

.language-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}
</style>