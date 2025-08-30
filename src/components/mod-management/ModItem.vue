<template>
  <n-descriptions :title="mod.name" column="4" label-align="left">
    <n-descriptions-item label="Size">
      <inline-code :display-text="bytes(occupation) ?? 'null'"/>
    </n-descriptions-item>
    <n-descriptions-item label="Hash">
      <inline-code :displayText="`${hash.slice(0,7)}...`" :src="hash"/>
    </n-descriptions-item>
  </n-descriptions>
</template>
<script lang="ts" setup>
import { BarotraumaMod } from "../../proto/mods.ts";
import { get_mod_hash, get_mod_occupation } from "../../invokes.ts";
import { onMounted, ref, Ref } from "vue";
import bytes from "bytes";
import InlineCode from "../utils/inlineCode.vue";

const props = defineProps<{
	mod: BarotraumaMod;
}>();

const occupation: Ref<number> = ref(0);

const hash: Ref<string> = ref("");

onMounted(async () => {
	occupation.value = await get_mod_occupation(props.mod.steamWorkshopId);

	hash.value = await get_mod_hash(props.mod.steamWorkshopId);
});
</script>
