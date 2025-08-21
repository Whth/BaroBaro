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

		// Set language preference
		if (config.value.uiConfig) {
			const languageMap: { [key: number]: string } = { 0: "en", 1: "zh" };
			const savedLanguage = languageMap[config.value.uiConfig.language] || "en";
			localStorage.setItem("language", savedLanguage);

			console.log("Config loaded successfully:", {
				theme: config.value.uiConfig.theme,
				language: savedLanguage,
				accentColor: config.value.uiConfig.accentColor,
			});
		} else {
			console.warn("No UI config found, using defaults");
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
		localStorage.setItem("language", "en");
		console.log("Using fallback defaults due to initialization error");

		// Don't rethrow - allow the app to continue even if initialization fails
	}
}

// Get stored language preference
export function getStoredLanguage(): "en" | "zh" {
	return (localStorage.getItem("language") as "en" | "zh") || "en";
}
