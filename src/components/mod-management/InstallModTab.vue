<template>
  <div class="install-mod-tab">
    <h2>Install Mod from Local File</h2>
    <div class="install-content">
      <div class="file-upload-section">
        <div
          class="file-drop-area"
          @dragover.prevent
          @drop.prevent="handleDrop"
        >
          <div class="file-drop-content">
            <p>Drag and drop mod files here</p>
            <p class="file-drop-subtext">or</p>
            <button class="browse-button" @click="browseFiles">
              Browse Files
            </button>
          </div>
        </div>
        <input
          ref="fileInput"
          type="file"
          multiple
          accept=".zip,.rar,.7z"
          class="file-input"
          @change="handleFileSelect"
        />
      </div>
      <div v-if="selectedFiles.length > 0" class="selected-files">
        <h3>Selected Files</h3>
        <ul class="file-list">
          <li
            v-for="(file, index) in selectedFiles"
            :key="index"
            class="file-item"
          >
            <span class="file-name">{{ file.name }}</span>
            <span class="file-size">{{ formatFileSize(file.size) }}</span>
          </li>
        </ul>
        <div class="install-actions">
          <button class="install-button" @click="installMods">
            Install Selected Mods
          </button>
          <button class="clear-button" @click="clearSelection">Clear</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { refreshInstalledMods } from "../../composables/useModManager";

const selectedFiles = ref<File[]>([]);
const fileInput = ref<HTMLInputElement | null>(null);

const browseFiles = () => {
	if (fileInput.value) {
		fileInput.value.click();
	}
};

const handleFileSelect = (event: Event) => {
	const target = event.target as HTMLInputElement;
	if (target.files) {
		selectedFiles.value = Array.from(target.files);
	}
};

const handleDrop = (event: DragEvent) => {
	if (event.dataTransfer?.files) {
		selectedFiles.value = Array.from(event.dataTransfer.files);
	}
};

const clearSelection = () => {
	selectedFiles.value = [];
	if (fileInput.value) {
		fileInput.value.value = "";
	}
};

const installMods = async () => {
	console.log("Installing mods:", selectedFiles.value);
	// In a real app, this would call the Tauri backend to install the mods
	try {
		// For now, we'll just simulate the installation and refresh the installed mods list
		alert(`Installing ${selectedFiles.value.length} mod(s)`);
		// Refresh the installed mods list after installation
		await refreshInstalledMods();
	} catch (error) {
		console.error("Failed to install mods:", error);
		alert("Failed to install mods.");
	}
};

const formatFileSize = (bytes: number) => {
	if (bytes === 0) return "0 Bytes";
	const k = 1024;
	const sizes = ["Bytes", "KB", "MB", "GB"];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return `${parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
};
</script>

<style scoped>
.install-mod-tab h2 {
  margin-top: 0;
  margin-bottom: var(--spacing-l);
}

.install-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-l);
}

.file-upload-section {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.file-drop-area {
  width: 100%;
  max-width: 500px;
  height: 200px;
  border: 2px dashed var(--color-border);
  border-radius: var(--border-radius-rounded);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: var(--spacing-m);
  cursor: pointer;
  transition: border-color 0.2s ease;
}

.file-drop-area:hover {
  border-color: var(--color-primary);
}

.file-drop-content {
  text-align: center;
  padding: var(--spacing-l);
}

.file-drop-content p {
  margin: 0 0 var(--spacing-s) 0;
  color: var(--color-text-primary);
}

.file-drop-subtext {
  color: var(--color-text-secondary);
  font-size: var(--font-size-body-small);
}

.browse-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.file-input {
  display: none;
}

.selected-files {
  width: 100%;
}

.selected-files h3 {
  margin: 0 0 var(--spacing-m) 0;
}

.file-list {
  list-style: none;
  padding: 0;
  margin: 0 0 var(--spacing-m) 0;
  max-height: 200px;
  overflow-y: auto;
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  background-color: var(--color-surface);
}

.file-item {
  display: flex;
  justify-content: space-between;
  padding: var(--spacing-s) var(--spacing-m);
  border-bottom: 1px solid var(--color-border);
}

.file-item:last-child {
  border-bottom: none;
}

.file-name {
  color: var(--color-text-primary);
  font-size: var(--font-size-body-regular);
}

.file-size {
  color: var(--color-text-secondary);
  font-size: var(--font-size-body-small);
}

.install-actions {
  display: flex;
  gap: var(--spacing-m);
}

.install-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}

.clear-button {
  padding: var(--spacing-s) var(--spacing-m);
  background-color: var(--color-surface);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-soft);
  cursor: pointer;
  font-weight: var(--font-weight-medium);
}
</style>
