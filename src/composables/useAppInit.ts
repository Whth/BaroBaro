import { ref } from "vue";
import { config, refresh_config } from "../invokes";
import { initializeApp } from "./useModManager";

export const isInitialized = ref(false);
export const initializationError = ref<string | null>(null);

export async function initializeApplication(): Promise<void> {
	if (isInitialized.value) {
		return; // Already initialized
	}

	try {
		// Load configuration first
		await refresh_config();

		// Set theme and language preferences from backend config
		if (config.value.uiConfig) {
			const themeMap: { [key: number]: string } = { 0: "dark", 1: "light" };
			const languageMap: { [key: number]: string } = { 0: "en", 1: "zh" };

			const savedTheme = themeMap[config.value.uiConfig.theme] || "light";
			const savedLanguage = languageMap[config.value.uiConfig.language] || "en";

			// Store both theme and language in localStorage for the theme system to use
			localStorage.setItem("theme", savedTheme);
			localStorage.setItem("language", savedLanguage);

			console.log("Config loaded successfully:", {
				theme: savedTheme,
				language: savedLanguage,
				accentColor: config.value.uiConfig.accentColor,
			});
		} else {
			console.warn("No UI config found, using defaults");
			// Set default values
			localStorage.setItem("theme", "light");
			localStorage.setItem("language", "en");
		}

		// Initialize the app with backend data
		await initializeApp();

		isInitialized.value = true;
		initializationError.value = null;
	} catch (error) {
		console.error("Failed to initialize app with backend data:", error);
		initializationError.value =
			error instanceof Error ? error.message : "Unknown initialization error";

		// Set fallback defaults
		localStorage.setItem("theme", "light");
		localStorage.setItem("language", "en");
		console.log("Using fallback defaults due to initialization error");

		// Don't rethrow - allow the app to continue even if initialization fails
	}
}

// Get stored language preference
export function getStoredLanguage(): "en" | "zh" {
	return (localStorage.getItem("language") as "en" | "zh") || "en";
}
