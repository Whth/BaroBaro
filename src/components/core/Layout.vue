<template>
  <n-layout class="layout" has-sider>
    <Navigation/>
    <n-layout>
      <n-layout-content class=main-content>
        <slot/>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script lang="ts" setup>
import Navigation from "./Navigation.vue";
</script>

