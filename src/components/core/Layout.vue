<template>
  <n-layout class="layout" has-sider style="height: 100vh">
    <Navigation/>
    <n-layout>
      <n-scrollbar>
        <n-layout-content style="padding: 3%">
          <slot/>
        </n-layout-content>
      </n-scrollbar>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import Navigation from "./Navigation.vue";
</script>

