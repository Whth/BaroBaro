<template>
  <footer class="app-footer">
    <div class="footer-content">
      <p class="copyright">© 2025 {{ t('app.title') }}. All rights reserved.</p>
      <div class="footer-links">
        <a href="#" class="footer-link">Documentation</a>
        <a href="#" class="footer-link">Support</a>
        <a href="#" class="footer-link">Version 0.1.0</a>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();
</script>

<style scoped>
.app-footer {
  background-color: var(--color-surface);
  border-top: 1px solid var(--color-border);
  padding: var(--spacing-s) var(--spacing-l);
  margin-top: auto;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  width: 250px;
}

.sidebar-footer {
  position: absolute;
  bottom: 0;
  left: 0;
  border-top: none;
  border-right: 1px solid var(--color-border);
}

.footer-content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--spacing-xs);
  max-width: 100%;
  margin: 0;
}

.copyright {
  margin: 0;
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.footer-links {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  align-items: flex-start;
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
