<template>
  <n-card :bordered="false" class="install-mod-card">
    <n-h2>Install Mod from Local File</n-h2>

    <!-- 文件上传区域 -->
    <n-upload
        v-model:file-list="fileList"
        accept=".zip,.rar,.7z"
        directory-dnd
        multiple
        @change="handleFileChange"
    >
      <n-upload-dragger>
        <div style="margin-bottom: 12px">
          <n-icon :depth="3" size="48">
            <ArchiveOutline/>
          </n-icon>
        </div>
        <n-text style="font-size: 16px">
          Drag and drop mod files here
        </n-text>
        <n-p depth="3" style="margin: 8px 0 0 0">
          or click to browse files
        </n-p>
      </n-upload-dragger>
    </n-upload>

    <!-- 已选择的文件 -->
    <div v-if="fileList.length > 0" style="margin-top: 20px">
      <n-h3>Selected Files</n-h3>

      <n-data-table
          :bordered="false"
          :columns="columns"
          :data="fileTableData"
          size="small"
      />

      <div style="margin-top: 20px">
        <n-space justify="end">
          <n-button @click="clearSelection">Clear</n-button>
          <n-button
              :loading="isInstalling"
              type="primary"
              @click="installMods"
          >
            Install Selected Mods
          </n-button>
        </n-space>
      </div>
    </div>
  </n-card>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import { ArchiveOutline } from "@vicons/ionicons5";
import type { UploadFileInfo } from "naive-ui";
import { refreshInstalledMods } from "../../composables/useModManager";

// 响应式数据
const fileList = ref<UploadFileInfo[]>([]);
const isInstalling = ref(false);

// 文件表格数据
const fileTableData = computed(() => {
	return fileList.value.map((file) => ({
		name: file.name,
		size: file.file ? formatFileSize(file.file.size) : "Unknown",
		file: file.file,
	}));
});

// 表格列配置
const columns = [
	{
		title: "File Name",
		key: "name",
		width: 200,
	},
	{
		title: "Size",
		key: "size",
		width: 100,
	},
];

// 处理文件变化
const handleFileChange = ({
	fileList: newFileList,
}: {
	fileList: UploadFileInfo[];
}) => {
	fileList.value = newFileList;
};

// 清空选择
const clearSelection = () => {
	fileList.value = [];
};

// 安装mods
const installMods = async () => {
	if (fileList.value.length === 0) return;

	isInstalling.value = true;
	try {
		// 获取实际文件对象
		const files = fileList.value
			.map((item) => item.file)
			.filter((file): file is File => file !== undefined);

		window.$message?.info(`Installing ${files.length} mod(s)`);

		// 模拟安装过程
		await new Promise((resolve) => setTimeout(resolve, 2000));

		window.$message?.success("Mods installed successfully!");
		await refreshInstalledMods();

		// 清空选择
		clearSelection();
	} catch (error) {
		window.$message?.error("Failed to install mods");
	} finally {
		isInstalling.value = false;
	}
};

// 格式化文件大小
const formatFileSize = (bytes: number) => {
	if (bytes === 0) return "0 Bytes";
	const k = 1024;
	const sizes = ["Bytes", "KB", "MB", "GB"];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return `${parseFloat((bytes / k ** i).toFixed(2))} ${sizes[i]}`;
};
</script>

<style scoped>
.install-mod-card {
  height: 100%;
}
</style>