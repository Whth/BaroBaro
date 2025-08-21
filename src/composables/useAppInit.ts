import { ref } from 'vue';
import { config, refresh_config } from '../invokes';
import { initializeApp } from './useModManager';

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
			localStorage.setItem('language', savedLanguage);
		}

		// Initialize the app with backend data
		await initializeApp();

		isInitialized.value = true;
		initializationError.value = null;
	} catch (error) {
		console.error("Failed to initialize app:", error);
		initializationError.value = error instanceof Error ? error.message : 'Unknown initialization error';
		// Don't rethrow - allow the app to continue even if initialization fails
	}
}

// Get stored language preference
export function getStoredLanguage(): "en" | "zh" {
	return (localStorage.getItem('language') as "en" | "zh") || "en";
}