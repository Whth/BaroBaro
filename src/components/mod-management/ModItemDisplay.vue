<template>
  <n-grid cols="10" x-gap="10px">
    <n-gi span="3">
      <n-image :src="mod.previewImage" width="100%"/>
    </n-gi>
    <n-gi
        span="7"
    >
      <n-descriptions column="4" label-align="left">
        <template #header>
          <n-h5>
            {{ mod.name }}
          </n-h5>
          <n-flex>
            <n-tag
                v-for="tag in mod.tags"
                :key="tag"
                :color="getTagColorConfig(tag)"
                :style="{

      cursor: 'pointer',
      transition: 'all 0.2s ease',
      opacity: '0.85',
    }"
                class="mod-tag"
                round
                size="medium"
                @click="openUrl(`https://steamcommunity.com/workshop/browse/?appid=602960&requiredtags[]=${encodeURIComponent(tag)}`)"
            >
              {{ tag }}
            </n-tag>
          </n-flex>
        </template>
        <n-descriptions-item :label="$t('modItem.occupation')">
          <inline-code :display-text="bytes(occupation) ?? 'null'"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modItem.version')">
          <inline-code :displayText="mod.modVersion"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modItem.gameVersion')">
          <inline-code :displayText="mod.gameVersion"/>
        </n-descriptions-item>
        <n-descriptions-item :label="$t('modItem.hash')">
          <inline-code :displayText="`${hash.slice(0,7)}...`" :src="hash"/>
        </n-descriptions-item>
      </n-descriptions>
    </n-gi>
  </n-grid>

</template>
<script lang="ts" setup>
import { BarotraumaMod } from "../../proto/mods.ts";
import {
	get_mod_hash,
	get_mod_occupation,
	retrieve_mod_metadata,
} from "../../invokes.ts";
import { onMounted, ref, Ref } from "vue";
import bytes from "bytes";
import InlineCode from "../utils/inlineCode.vue";
import getTagColorConfig from "../../composables/coloredTag.ts";
import { openUrl } from "@tauri-apps/plugin-opener";

const props = defineProps<{
	mod: BarotraumaMod;
}>();

const occupation: Ref<number> = ref(0);

const hash: Ref<string> = ref("");
onMounted(async () => {
	const loadOccupation = async () => {
		occupation.value = await get_mod_occupation(props.mod.steamWorkshopId);
	};

	const loadHash = async () => {
		hash.value = await get_mod_hash(props.mod.steamWorkshopId);
	};

	await Promise.all([
		retrieve_mod_metadata(), // Runs concurrently, no assignment needed
		loadOccupation(), // Runs and assigns
		loadHash(), // Runs and assigns
	]);
});
</script>
