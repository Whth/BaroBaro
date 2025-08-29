import { Theme } from "../proto/config.ts";
import type { GlobalTheme, GlobalThemeOverrides } from "naive-ui";
import { darkTheme, lightTheme } from "naive-ui";
import { type Ref, ref } from "vue";
import { config, get_background_image } from "../invokes.ts";

const opc = (a: number) => {
	return `rgba(255, 255, 255, ${a})`;
};

const darkOpc = (a: number) => {
	return `rgba(36, 36, 36, ${a})`;
};

const darkOverrides: Ref<GlobalThemeOverrides> = ref({
	common: {
		bodyColor: darkOpc(0.4),
		cardColor: darkOpc(0.4),
		tabColor: darkOpc(0.4),
	},
});

const lightOverrides: Ref<GlobalThemeOverrides> = ref({
	common: {
		bodyColor: opc(0.4),
		cardColor: opc(0.4),
		tabColor: opc(0.4),
	},
});

export function setTheme() {
	const theme = config.value.uiConfig?.theme ?? Theme.Light;
	currentTheme.value = theme_mapping[theme];
	currentThemeOverrides.value = overridesMapping[theme].value;
}

export function setTransparent() {
	const a = config.value.uiConfig?.backgroundOpacity ?? 0.4;
	darkOverrides.value = {
		common: {
			bodyColor: darkOpc(a),
			cardColor: darkOpc(a),
			tabColor: darkOpc(a),
		},
	};
	lightOverrides.value = {
		common: {
			bodyColor: opc(a),
			cardColor: opc(a),
			tabColor: opc(a),
		},
	};
}

export async function setBackgroundImage() {
	currentBackgroundImage.value = await get_background_image();
}

export const theme_mapping = {
	[Theme.Dark]: darkTheme,
	[Theme.Light]: lightTheme,
	[Theme.UNRECOGNIZED]: lightTheme,
};

export const overridesMapping = {
	[Theme.Dark]: darkOverrides,
	[Theme.Light]: lightOverrides,
	[Theme.UNRECOGNIZED]: lightOverrides,
};

export const currentTheme: Ref<GlobalTheme> = ref(theme_mapping[Theme.Light]);

export const currentThemeOverrides: Ref<GlobalThemeOverrides> = ref(
	overridesMapping[Theme.Light],
);

export const currentBackgroundImage: Ref<string | null> = ref(null);
