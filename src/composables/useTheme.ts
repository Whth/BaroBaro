import {Theme} from "../proto/config.ts"
import {darkTheme, lightTheme} from "naive-ui"
import {ref} from "vue";

export const theme_mapping = {
    [Theme.Dark]: darkTheme,
    [Theme.Light]: lightTheme,
    [Theme.UNRECOGNIZED]: lightTheme,
};

export const currentTheme = ref(theme_mapping[Theme.Light]);