<template>
  <component :is="tag" :class="titleClass">
    <slot />
  </component>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  type?: "page" | "section" | "card";
}>();

const tag = computed(() => {
  if (props.level) {
    return `h${props.level}`;
  }
  switch (props.type) {
    case "page":
      return "h1";
    case "section":
      return "h2";
    case "card":
      return "h3";
    default:
      return "h1";
  }
});

const titleClass = computed(() => {
  switch (props.type) {
    case "page":
      return "page-title";
    case "section":
      return "section-title";
    case "card":
      return "card-title";
    default:
      return "page-title";
  }
});
</script>

<style scoped>
/* Styles are defined in the global styles.css file */
</style>