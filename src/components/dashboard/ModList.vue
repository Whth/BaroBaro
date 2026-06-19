<template>
  <n-scrollbar style="height: 69vh">
    <draggable
        v-model="enabled_mods"
        handle=".drag-handle"
        item-key="steamWorkshopId"
        @change="onOrderChanged"
    >
      <template #item="{ element, index }">
        <div style="display: flex; align-items: center; margin-bottom: 12px;">
          <div class="drag-handle" style="cursor: grab; padding: 0 8px 0 0;">
            <n-icon size="20">
              <menu-outline/>
            </n-icon>
          </div>
          <div style="flex: 1;">
            <ModCard :index="index + 1" :mod="element" @mod-selected="viewMod"/>
          </div>
        </div>
      </template>
    </draggable>
    <n-empty v-if="enabled_mods.length === 0" :description="$t('modList.empty')" style="margin-top: 20px">
      <template #icon>
        <n-icon>
          <archive-outline/>
        </n-icon>
      </template>
    </n-empty>
  </n-scrollbar>
  <div v-if="orderChanged" style="margin-top: 12px; display: flex; justify-content: flex-end;">
    <n-button :loading="saving" type="primary" @click="saveOrder">
      {{ $t('modList.saveOrder') }}
    </n-button>
  </div>
</template>


<script lang="ts" setup>
import {
	clear_active_profile,
	enabled_mods,
	list_enabled_mods,
	reorder_enabled_mods,
} from "../../invokes.ts";
import ModCard from "./ModCard.vue";
import type { BarotraumaMod } from "../../proto/mods.ts";
import { ArchiveOutline, MenuOutline } from "@vicons/ionicons5";
import draggable from "vuedraggable";
import { ref } from "vue";
import { useMessage } from "naive-ui";
import { useI18n } from "vue-i18n";

const emit = defineEmits(["viewingMod"]);
const message = useMessage();
const { t } = useI18n();

const orderChanged = ref(false);
const saving = ref(false);

function viewMod(mod: BarotraumaMod) {
	emit("viewingMod", mod);
}

function onOrderChanged() {
	orderChanged.value = true;
}

async function saveOrder() {
	saving.value = true;
	try {
		const orderedIds = enabled_mods.value.map((m) => m.steamWorkshopId);
		await reorder_enabled_mods(orderedIds);
		await list_enabled_mods();
		await clear_active_profile();
		orderChanged.value = false;
		message.success(t("modList.orderSaved"));
	} catch (error) {
		message.error(String(error));
	} finally {
		saving.value = false;
	}
}
</script>
