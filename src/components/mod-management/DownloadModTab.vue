<template>
  <div class="download-mod-tab">
    <h2>Download Mods from Steam Workshop</h2>
    <div class="download-content">
      <div class="search-section">
        <div class="search-container">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search for mods on Steam Workshop..."
            class="search-input"
          />
          <button class="search-button">🔍</button>
        </div>
        <div class="filter-options">
          <select v-model="categoryFilter" class="filter-select">
            <option value="all">All Categories</option>
            <option value="graphics">Graphics</option>
            <option value="audio">Audio</option>
            <option value="gameplay">Gameplay</option>
            <option value="utility">Utility</option>
          </select>
          <select v-model="sortBy" class="filter-select">
            <option value="popular">Most Popular</option>
            <option value="newest">Newest</option>
            <option value="rating">Highest Rated</option>
          </select>
        </div>
      </div>
      <div class="mod-results">
        <div v-if="loading" class="loading">
          Loading mods from Steam Workshop...
        </div>
        <div v-else-if="mods.length === 0" class="no-results">
          No mods found
        </div>
        <div v-else class="mod-grid">
          <div v-for="mod in mods" :key="mod.id" class="mod-card">
            <div class="mod-card-header">
              <h3 class="mod-name">{{ mod.name }}</h3>
              <span class="mod-version">{{ mod.version }}</span>
            </div>
            <p class="mod-description">{{ mod.description }}</p>
            <div class="mod-meta">
              <span class="mod-author">by {{ mod.author }}</span>
              <span class="mod-downloads">📥 {{ mod.downloadCount }}</span>
            </div>
            <div class="mod-actions">
              <button
                class="download-button"
                :disabled="mod.downloading"
                @click="downloadMod(mod.id)"
              >
                {{ mod.downloading ? "Downloading..." : "Download from Steam" }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { downloadMods } from "../../composables/useModManager";

// This component is prepared for SteamCMD integration
// It will connect to the existing SteamCMD Rust crate in the Tauri backend

interface Mod {
	id: string;
	name: string;
	version: string;
	author: string;
	description: string;
	downloadCount: number;
	category: string;
	rating: number;
	downloading?: boolean;
}

const _searchQuery = ref("");
const _categoryFilter = ref("all");
const _sortBy = ref("popular");
const loading = ref(false);

const mods = ref<Mod[]>([
	{
		id: "1",
		name: "Enhanced Graphics Pack",
		version: "2.1.0",
		author: "VisualMaster",
		description: "High resolution textures and improved lighting effects",
		downloadCount: 125000,
		category: "graphics",
		rating: 4.8,
	},
	{
		id: "2",
		name: "Immersive Soundscapes",
		version: "1.5.3",
		author: "AudioWizard",
		description: "Realistic ambient sounds and improved audio effects",
		downloadCount: 87500,
		category: "audio",
		rating: 4.6,
	},
	{
		id: "3",
		name: "Advanced Crafting System",
		version: "3.0.1",
		author: "CraftExpert",
		description: "Revamped crafting mechanics with new recipes and items",
		downloadCount: 210000,
		category: "gameplay",
		rating: 4.9,
	},
]);

const _downloadMod = async (modId: string) => {
	const mod = mods.value.find((m) => m.id === modId);
	if (mod) {
		mod.downloading = true;
		console.log(`Downloading mod from Steam Workshop: ${mod.name}`);
		try {
			// In a real app, this would integrate with the SteamCMD Rust crate
			// For now, we'll call the downloadMods function from the composable
			await downloadMods([parseInt(modId, 10)]);
			mod.downloading = false;
			alert(`Downloaded ${mod.name} from Steam Workshop successfully!`);
		} catch (error) {
			mod.downloading = false;
			console.error("Failed to download mod:", error);
			alert(`Failed to download ${mod.name} from Steam Workshop.`);
		}
	}
};

onMounted(() => {
	// Simulate loading
	loading.value = true;
	setTimeout(() => {
		loading.value = false;
	}, 1000);
});
</script>

<style scoped>
.download-mod-tab h2 {
  margin-top: 0;
  margin-bottom: var(--spacing-l);
}

.download-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-l);
}

.search-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-m);
}

.search-container {
  display: flex;
  align-items: center;
  max-width: 500px;
}

.search-input {
  flex: 1;
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.search-button {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-left: none;
  border-radius: var(--border-radius-soft);
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  cursor: pointer;
}

.filter-options {
  display: flex;
  gap: var(--spacing-m);
}

.filter-select {
  padding: var(--spacing-s) var(--spacing-m);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.mod-results {
  flex: 1;
}

.loading,
.no-results {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-secondary);
}

.mod-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--spacing-m);
}

.mod-card {
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-rounded);
  padding: var(--spacing-m);
  background-color: var(--color-surface);
  transition: all 0.2s ease;
}

.mod-card:hover {
  box-shadow: var(--shadow-level-2);
  border-color: var(--color-primary-light);
}

.mod-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-s);
}

.mod-name {
  margin: 0;
  font-size: var(--font-size-heading-3);
  color: var(--color-text-primary);
}

.mod-version {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.mod-description {
  margin: 0 0 var(--spacing-m) 0;
  color: var(--color-text-primary);
  line-height: 1.4;
}

.mod-meta {
  display: flex;
  justify-content: space-between;
  margin-bottom: var(--spacing-m);
}

.mod-author {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.mod-downloads {
  font-size: var(--font-size-body-small);
  color: var(--color-text-secondary);
}

.mod-actions {
  display: flex;
  justify-content: flex-end;
}

.download-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.download-button:disabled {
  background-color: var(--color-text-secondary);
  cursor: not-allowed;
}
</style>
