// Import locale messages
import en from "./locales/en.json";
import zh from "./locales/zh.json";
import { Language, languageToJSON } from "./proto/config.ts";
import { createI18n } from "vue-i18n";
import { config } from "./invokes.ts";

const messages = {
	[languageToJSON(Language.EN)]: en,
	[languageToJSON(Language.ZH)]: zh,
};

export const i18n = createI18n({
	legacy: false,
	locale: languageToJSON(config.value.uiConfig?.language ?? Language.EN),
	globalInjection: true,
	messages: messages,
});
