import { createI18n } from "vue-i18n";

// Import locale messages
import en from "./locales/en.json";
import zh from "./locales/zh.json";

export type MessageSchema = typeof en;

const messages = {
	en: en,
	zh: zh,
};

// Get initial language from localStorage or default to 'en'
const getInitialLanguage = (): "en" | "zh" => {
	if (typeof window !== 'undefined') {
		return (localStorage.getItem('language') as "en" | "zh") || "en";
	}
	return "en";
};

const i18n = createI18n<[MessageSchema], "en" | "zh">({
	legacy: false, // Use Composition API
	locale: getInitialLanguage(), // Get from localStorage or default
	fallbackLocale: "en", // Fallback locale
	messages,
	globalInjection: true, // Allow $t in templates
});

export default i18n;
