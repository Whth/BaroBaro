import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import i18n from "./i18n";
import "./assets/styles.css";
import { initializeApp } from "./composables/useModManager";
import { config, refresh_config } from "./invokes";

// Load language preference from backend config
let savedLanguage = "en";
try {
	await refresh_config();
	if (config.value.uiConfig) {
		const languageMap: { [key: number]: string } = { 0: "en", 1: "zh" };
		savedLanguage = languageMap[config.value.uiConfig.language] || "en";
	}
} catch (e) {
	console.error("Failed to load language config from backend", e);
}

i18n.global.locale = savedLanguage as "en" | "zh";

// Initialize the app with backend data
initializeApp()
	.then(() => {
		const app = createApp(App);
		app.use(router);
		app.use(i18n);
		app.mount("#app");
	})
	.catch((error) => {
		console.error("Failed to initialize app:", error);
		// Still mount the app even if initialization fails
		const app = createApp(App);
		app.use(router);
		app.use(i18n);
		app.mount("#app");
	});
