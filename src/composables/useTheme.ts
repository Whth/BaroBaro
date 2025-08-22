import { ref, computed, onMounted, watch } from 'vue'
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

    // Update naive-ui theme immediately
    updateNaiveUITheme(mode)

    // Update CSS variables for custom theme
    updateCSSVariables(mode)

    // Force a re-render by dispatching a custom event
    window.dispatchEvent(new CustomEvent('theme-changed', {
      detail: { theme: mode }
    }))
  }

  const updateCSSVariables = (mode: ThemeMode) => {
    const root = document.documentElement

    if (mode === 'dark') {
      root.style.setProperty('--color-background', 'var(--color-background-dark)')
      root.style.setProperty('--color-surface', 'var(--color-surface-dark)')
      root.style.setProperty('--color-text-primary', 'var(--color-text-primary-dark)')
      root.style.setProperty('--color-text-secondary', 'var(--color-text-secondary-dark)')
      root.style.setProperty('--color-border', 'var(--color-border-dark)')
      root.setAttribute('data-theme', 'dark')
    } else {
      root.style.setProperty('--color-background', 'var(--color-background-light)')
      root.style.setProperty('--color-surface', 'var(--color-surface-light)')
      root.style.setProperty('--color-text-primary', 'var(--color-text-primary-light)')
      root.style.setProperty('--color-text-secondary', 'var(--color-text-secondary-light)')
      root.style.setProperty('--color-border', 'var(--color-border-light)')
      root.setAttribute('data-theme', 'light')
    }

    // Load background image from config via Tauri
    loadBackgroundImage()
  }

  const loadBackgroundImage = async () => {
    try {
      // Import here to avoid circular dependencies
      const { get_background_image } = await import('../invokes')

      const backgroundDataUrl = await get_background_image()

      if (backgroundDataUrl) {
        document.documentElement.style.setProperty('--background-image', backgroundDataUrl)
        // Update localStorage for consistency
        localStorage.setItem('backgroundImage', backgroundDataUrl)
        console.log('Background image loaded from config')
      } else {
        document.documentElement.style.setProperty('--background-image', 'none')
        localStorage.removeItem('backgroundImage')
        console.log('No background image configured')
      }
    } catch (error) {
      console.error('Failed to load background image:', error)
      document.documentElement.style.setProperty('--background-image', 'none')
      localStorage.removeItem('backgroundImage')
    }
  }

  const toggleTheme = () => {
    const newTheme = themeMode.value === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
  }

  const getStoredTheme = (): ThemeMode => {
    return (localStorage.getItem('theme') as ThemeMode) || 'light'
  }

  const updateNaiveUITheme = (mode: ThemeMode) => {
    // This will trigger the reactive update of naiveTheme computed property
    themeMode.value = mode

    // Force a re-render by dispatching a custom event
    window.dispatchEvent(new CustomEvent('theme-changed', {
      detail: { theme: mode }
    }))
  }

  // Watch for theme changes and apply them
  watch(themeMode, (newTheme) => {
    updateNaiveUITheme(newTheme)
  })

  onMounted(() => {
    const storedTheme = getStoredTheme()
    setTheme(storedTheme)
  })

  // Initialize CSS variables on first load
  const initializeTheme = () => {
    const storedTheme = getStoredTheme()
    updateCSSVariables(storedTheme)
  }

  // Call initialization
  initializeTheme()

  return {
    themeMode: computed(() => themeMode.value),
    naiveTheme,
    setTheme,
    toggleTheme,
    getStoredTheme
  }
}