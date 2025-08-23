import {createApp} from "vue";
import App from "./App.vue";
import router from "./router";
import {i18n} from "./i18n";
import "./assets/styles.css";

// Naive UI setup
import {
    create,
    NAlert,
    NButton,
    NCard,
    NColorPicker,
    NConfigProvider,
    NDataTable,
    NDialogProvider,
    NDivider,
    NDrawer,
    NForm,
    NFormItem,
    NH1,
    NH2,
    NH3,
    NIcon,
    NInput,
    NLayout,
    NLayoutContent,
    NLayoutFooter,
    NLayoutHeader,
    NLayoutSider,
    NLoadingBarProvider,
    NMenu,
    NMessageProvider,
    NModal,
    NNotificationProvider,
    NProgress,
    NSelect,
    NSlider,
    NSpace,
    NSpin,
    NSwitch,
    NTable,
    NTabPane,
    NTabs,
    NText,
} from "naive-ui";

const naive = create({
    components: [
        NH1,
        NH2,
        NH3,
        NColorPicker,
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


createApp(App)
    .use(router)
    .use(i18n)
    .use(naive)
    .mount("#app");
