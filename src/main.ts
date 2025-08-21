import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import i18n from "./i18n";
import "./assets/styles.css";
import { initializeApp } from "./composables/useModManager";

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
