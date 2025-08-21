import { createI18n } from 'vue-i18n'

// Import locale messages
import en from '../public/locales/en.json'

export type MessageSchema = typeof en

const messages = {
  en: en
}

const i18n = createI18n<[MessageSchema], 'en'>({
  legacy: false, // Use Composition API
  locale: 'en', // Default locale
  fallbackLocale: 'en', // Fallback locale
  messages,
  globalInjection: true, // Allow $t in templates
})

export default i18n