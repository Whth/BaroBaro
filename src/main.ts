import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { i18n } from "./i18n";
import "./assets/styles.css";

import * as naive from "naive-ui";

createApp(App).use(router).use(i18n).use(naive).mount("#app");
