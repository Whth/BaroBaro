<template>
  <div class="layout">
    <Header />
    <div class="layout-content">
      <Navigation />
      <main class="main-content">
        <slot />
      </main>
    </div>
    <Footer />
  </div>
</template>

<script setup lang="ts">
import Header from './Header.vue'
import Navigation from './Navigation.vue'
import Footer from './Footer.vue'
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