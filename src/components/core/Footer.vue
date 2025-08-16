<template>
  <footer class="app-footer">
    <div class="footer-content">
      <p class="copyright">© 2025 Game Mod Manager. All rights reserved.</p>
      <div class="footer-links">
        <a href="#" class="footer-link">Documentation</a>
        <a href="#" class="footer-link">Support</a>
        <a href="#" class="footer-link">Version 0.1.0</a>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
</script>

<style scoped>
.app-footer {
  background-color: var(--color-surface);
  border-top: 1px solid var(--color-border);
  padding: var(--spacing-s) var(--spacing-l);
  margin-top: auto;
}

.footer-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.copyright {
  margin: 0;
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.footer-links {
  display: flex;
  gap: var(--spacing-m);
}

.footer-link {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
  text-decoration: none;
}

.footer-link:hover {
  color: var(--color-primary);
}
</style>