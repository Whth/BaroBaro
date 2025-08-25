<template>
  <n-grid v-if="mod">
    <n-image :src="mod.previewImage">

    </n-image>
  </n-grid>
  <n-h4 v-if="!mod" v-text="$t('modDetails.notSelected')"></n-h4>

</template>


<script lang="ts" setup>
import type {BarotraumaMod} from '../../proto/mods.ts'


interface Props {
  mod: BarotraumaMod | null,
}

defineProps<Props>()
</script>