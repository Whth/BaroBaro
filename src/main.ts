import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import i18n from "./i18n";
import "./assets/styles.css";

// Naive UI setup
import {
	create,
	NButton,
	NCard,
	NLayout,
	NLayoutHeader,
	NLayoutContent,
	NLayoutFooter,
	NLayoutSider,
	NMenu,
	NIcon,
	NText,
	NSpace,
	NInput,
	NSelect,
	NForm,
	NFormItem,
	NSwitch,
	NSlider,
	NProgress,
	NSpin,
	NModal,
	NDrawer,
	NTable,
	NDataTable,
	NTabs,
	NTabPane,
	NDivider,
	NAlert,
	NMessageProvider,
	NConfigProvider,
	NDialogProvider,
	NNotificationProvider,
	NLoadingBarProvider,
} from "naive-ui";

const naive = create({
	components: [
		NButton,
		NCard,
		NLayout,
		NLayoutHeader,
		NLayoutContent,
		NLayoutFooter,
		NLayoutSider,
		NMenu,
		NIcon,
		NText,
		NSpace,
		NInput,
		NSelect,
		NForm,
		NFormItem,
		NSwitch,
		NSlider,
		NProgress,
		NSpin,
		NModal,
		NDrawer,
		NTable,
		NDataTable,
		NTabs,
		NTabPane,
		NDivider,
		NAlert,
		NMessageProvider,
		NConfigProvider,
		NDialogProvider,
		NNotificationProvider,
		NLoadingBarProvider,
	],
});

const app = createApp(App);
app.use(router);
app.use(i18n);
app.use(naive);
app.mount("#app");
