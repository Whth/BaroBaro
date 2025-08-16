<template>
  <div class="profiles-view">
    <h1>Profile Management</h1>
    <div class="profiles-content">
      <ProfileList />
      <ProfileEditor />
    </div>
  </div>
</template>

<script setup lang="ts">
import ProfileList from '@/components/profile-management/ProfileList.vue'
import ProfileEditor from '@/components/profile-management/ProfileEditor.vue'
</script>

<style scoped>
.profiles-view {
  padding: var(--spacing-l);
}

.profiles-view h1 {
  margin: 0 0 var(--spacing-l) 0;
}

.profiles-content {
  display: flex;
  gap: var(--spacing-l);
}

@media (max-width: 768px) {
  .profiles-content {
    flex-direction: column;
  }
}
</style>