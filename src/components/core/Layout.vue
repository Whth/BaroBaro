<template>
  <div class="layout">
    <Header class="acrylic" />
    <div class="layout-content">
      <Navigation class="acrylic" />
      <main class="main-content">
        <slot />
      </main>
    </div>
    <Footer class="acrylic" />
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped>
.layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.layout-content {
  display: flex;
  flex: 1;
}

.main-content {
  flex: 1;
  padding: var(--spacing-l);
  overflow-y: auto;
}
</style>
