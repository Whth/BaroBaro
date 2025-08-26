import {createApp} from "vue";
import App from "./App.vue";
import router from "./router";
import {i18n} from "./i18n";
import "./assets/styles.css";

import * as naive from "naive-ui";
import VueBbob from '@bbob/vue3';

createApp(App).use(router).use(i18n).use(naive)
    .use(VueBbob).mount("#app");
