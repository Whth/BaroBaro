<template>
  <n-layout class="layout" has-sider style="height: 100vh">
    <Navigation/>
    <n-layout>
      <n-scrollbar>
        <n-layout-content style="padding-left: 3%;padding-right: 3%;height: 100vh">
          <slot/>
        </n-layout-content>
      </n-scrollbar>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import Navigation from "./Navigation.vue";
</script>

