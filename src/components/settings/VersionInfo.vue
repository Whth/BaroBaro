<template>
  <div class="version-info">
    <h2>Version Information</h2>
    <div class="info-cards">
      <div class="info-card">
        <h3>🎮 Game Mod Manager</h3>
        <div class="version-details">
          <p><strong>Version:</strong> 0.1.0</p>
          <p><strong>Build:</strong> Alpha</p>
          <p><strong>Release Date:</strong> 2025</p>
        </div>
      </div>

      <div class="info-card">
        <h3>📋 About</h3>
        <p>A modern, user-friendly mod manager for Barotrauma game.</p>
        <p>Built with Vue.js and Tauri for cross-platform compatibility.</p>
      </div>

      <div class="info-card">
        <h3>🔗 Links & Support</h3>
        <div class="link-list">
          <a href="#" class="info-link">📚 Documentation</a>
          <a href="#" class="info-link">🆘 Support</a>
          <a href="#" class="info-link">🐛 Report Issues</a>
        </div>
      </div>

      <div class="info-card">
        <h3>© Copyright</h3>
        <p>© 2025 Game Mod Manager. All rights reserved.</p>
        <p>This software is provided as-is without warranty.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Version info component - displays app version, copyright, and support links
</script>

<style scoped>
.version-info h2 {
  margin: 0 0 var(--spacing-l) 0;
  color: var(--color-text-primary);
  font-size: var(--font-size-heading-2);
}

.info-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-l);
}

.info-card {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  padding: var(--spacing-l);
  transition: all 0.3s ease;
}

.info-card:hover {
  border-color: var(--color-primary);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.info-card h3 {
  margin: 0 0 var(--spacing-m) 0;
  color: var(--color-text-primary);
  font-size: var(--font-size-heading-3);
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
}

.version-details {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.version-details p {
  margin: 0;
  color: var(--color-text-secondary);
  font-size: var(--font-size-body-regular);
}

.version-details strong {
  color: var(--color-text-primary);
}

.info-card p {
  margin: 0 0 var(--spacing-s) 0;
  color: var(--color-text-secondary);
  line-height: 1.5;
}

.info-card p:last-child {
  margin-bottom: 0;
}

.link-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-s);
}

.info-link {
  display: flex;
  align-items: center;
  gap: var(--spacing-s);
  padding: var(--spacing-s);
  color: var(--color-text-primary);
  text-decoration: none;
  border-radius: var(--border-radius-soft);
  transition: all 0.3s ease;
  font-size: var(--font-size-body-regular);
}

.info-link:hover {
  background-color: var(--color-background);
  color: var(--color-primary);
  transform: translateX(4px);
}

@media (max-width: 768px) {
  .info-cards {
    grid-template-columns: 1fr;
  }

  .info-card {
    padding: var(--spacing-m);
  }
}
</style>