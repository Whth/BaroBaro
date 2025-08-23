<template>
  <n-layout class="layout" has-sider style="height: 100vh">
    <Navigation/>
    <n-layout>
      <n-layout-content style="padding: 3%">
        <slot/>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import Navigation from "./Navigation.vue";
</script>

