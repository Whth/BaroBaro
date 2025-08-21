import { ref, computed, onMounted } from 'vue'
import { lightTheme, darkTheme } from 'naive-ui'
import type { GlobalTheme } from 'naive-ui'

export type ThemeMode = 'light' | 'dark'

const themeMode = ref<ThemeMode>('light')

export function useTheme() {
  const naiveTheme = computed<GlobalTheme>(() =>
    themeMode.value === 'dark' ? darkTheme : lightTheme
  )

  const setTheme = (mode: ThemeMode) => {
    themeMode.value = mode
    localStorage.setItem('theme', mode)

    // Apply theme to document for custom CSS
    document.documentElement.setAttribute('data-theme', mode)
  }

  const toggleTheme = () => {
    const newTheme = themeMode.value === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
  }

  const getStoredTheme = (): ThemeMode => {
    return (localStorage.getItem('theme') as ThemeMode) || 'light'
  }

  onMounted(() => {
    const storedTheme = getStoredTheme()
    setTheme(storedTheme)
  })

  return {
    themeMode: computed(() => themeMode.value),
    naiveTheme,
    setTheme,
    toggleTheme,
    getStoredTheme
  }
}