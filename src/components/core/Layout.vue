<template>
   <n-layout class="layout" has-sider>
     <Navigation />
     <n-layout>
       <n-layout-content class="main-content">
         <slot />
       </n-layout-content>
     </n-layout>
   </n-layout>
 </template>

<script setup lang="ts">
import Navigation from "./Navigation.vue";
</script>

<style scoped>
.layout {
  min-height: 100vh;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-content {
  padding: 24px;
  overflow-y: auto;
  background: transparent;
  flex: 1;
  position: relative;
}

:deep(.n-layout) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.n-layout-content) {
  flex: 1;
  display: flex;
  flex-direction: column;
}
</style>
