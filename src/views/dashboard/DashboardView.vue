<template>


  <TitledPage>
    <template #title>
      <n-h1 v-text="$t('navigation.dashboard')"></n-h1>
    </template>
    <n-card>
      <SearchAndFilter/>
    </n-card>

    <n-grid cols="12" x-gap="24" y-gap="24">
      <n-grid-item span="8">
        <n-card title="Mods List">
          <ModList/>
        </n-card>
      </n-grid-item>

      <n-grid-item span="4">
        <n-card title="Mod Details">
          <ModDetails :selectedMod="currentMod" @remove="onRemove" @toggle="onToggle" @update="onUpdate"/>
        </n-card>
      </n-grid-item>
    </n-grid>
  </TitledPage>

</template>

<script lang="ts" setup>
import SearchAndFilter from "../../components/dashboard/SearchAndFilter.vue";
import ModList from "../../components/dashboard/ModList.vue";
import TitledPage from "../../components/core/TitledPage.vue";
import type {BarotraumaMod} from '../../proto/mods'
import {onMounted, ref} from "vue";
import ModDetails from "../../components/dashboard/ModDetails.vue";
import {list_enabled_mods, list_installed_mods, list_mod_lists} from "../../invokes.ts";


onMounted(async () => {
  await Promise.all([
    list_installed_mods(),
    list_enabled_mods(),
    list_mod_lists()
  ])
})

// 假设这是当前选中的 mod
const currentMod = ref<BarotraumaMod | null>(null)

// 事件处理
const onToggle = (mod: BarotraumaMod) => { /* ... */
}
const onUpdate = (mod: BarotraumaMod) => { /* ... */
}
const onRemove = (mod: BarotraumaMod) => { /* ... */
}
</script>


