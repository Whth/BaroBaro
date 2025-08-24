import { Theme } from "../proto/config.ts";
import type { GlobalTheme } from "naive-ui";
import { darkTheme, lightTheme } from "naive-ui";
import { type Ref, ref } from "vue";

export const theme_mapping = {
	[Theme.Dark]: darkTheme,
	[Theme.Light]: lightTheme,
	[Theme.UNRECOGNIZED]: lightTheme,
};

export const currentTheme: Ref<GlobalTheme> = ref(theme_mapping[Theme.Light]);
