<template>
  <n-text code @click="copySrc(src)">{{ src }}</n-text>
</template>

<script lang="ts" setup>
import useClipboard from "vue-clipboard3";
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";

interface Props {
	src: string;
}

defineProps<Props>();

const { toClipboard } = useClipboard();
const message = useMessage();
const { t } = useI18n();

const copySrc = (src: string) => {
	toClipboard(src);
	message.success(t("code.copied"));
};
</script>
