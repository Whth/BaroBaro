<template>
  <n-card :bordered="false" class="update-mod-card">
    <n-h2>Update Mods</n-h2>

    <n-result
        v-if="modsToUpdate.length === 0"
        description="No updates available at this time."
        status="success"
        title="All mods are up to date!"
    />

    <div v-else>
      <!-- 头部信息 -->
      <div class="update-header">
        <n-text>{{ modsToUpdate.length }} mod(s) have available updates</n-text>
        <n-button
            :loading="isUpdatingAll"
            secondary
            type="primary"
            @click="updateAllMods"
        >
          Update All
        </n-button>
      </div>

      <!-- 更新列表 -->
      <n-list>
        <n-list-item v-for="mod in modsToUpdate" :key="mod.id">
          <n-thing>
            <template #header>
              <n-ellipsis style="max-width: 200px">
                {{ mod.name }}
              </n-ellipsis>
            </template>

            <template #description>
              <n-space align="center" size="small">
                <n-tag type="default">{{ mod.currentVersion }}</n-tag>
                <n-icon color="#666" size="16">
                  <ArrowForward/>
                </n-icon>
                <n-tag type="success">{{ mod.newVersion }}</n-tag>
              </n-space>
              <n-ellipsis style="margin-top: 4px; max-width: 300px">
                {{ mod.description }}
              </n-ellipsis>
            </template>

            <template #actions>
              <n-space>
                <n-button
                    :loading="mod.updating"
                    size="small"
                    type="primary"
                    @click="updateMod(mod.id)"
                >
                  {{ mod.updating ? 'Updating...' : 'Update' }}
                </n-button>
                <n-button
                    secondary
                    size="small"
                    @click="viewChangelog(mod.id)"
                >
                  Changelog
                </n-button>
              </n-space>
            </template>
          </n-thing>
        </n-list-item>
      </n-list>
    </div>
  </n-card>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { ArrowForward } from "@vicons/ionicons5";

// 定义接口
interface ModUpdate {
	id: string;
	name: string;
	currentVersion: string;
	newVersion: string;
	description: string;
	updating?: boolean;
}

// 响应式数据
const modsToUpdate = ref<ModUpdate[]>([
	{
		id: "1",
		name: "Better Graphics",
		currentVersion: "1.2.3",
		newVersion: "1.3.0",
		description: "Enhanced textures and lighting improvements",
	},
	{
		id: "2",
		name: "New Weapons Pack",
		currentVersion: "2.0.1",
		newVersion: "2.1.0",
		description: "Added 10 new weapons and balanced existing ones",
	},
]);

const isUpdatingAll = ref(false);

// 更新单个mod
const updateMod = async (modId: string) => {
	const mod = modsToUpdate.value.find((m) => m.id === modId);
	if (!mod) return;

	mod.updating = true;
	try {
		// 模拟更新过程
		await new Promise((resolve) => setTimeout(resolve, 2000));

		// 更新成功后从列表中移除
		modsToUpdate.value = modsToUpdate.value.filter((m) => m.id !== modId);

		window.$message?.success(`Updated ${mod.name} successfully!`);
		await refreshInstalledMods();
	} catch (error) {
		window.$message?.error(`Failed to update ${mod.name}`);
	} finally {
		mod.updating = false;
	}
};

// 更新所有mods
const updateAllMods = async () => {
	isUpdatingAll.value = true;
	try {
		// 并行更新所有mods
		const updatePromises = modsToUpdate.value.map((mod) => updateMod(mod.id));
		await Promise.all(updatePromises);
		window.$message?.success("All mods updated successfully!");
	} catch (error) {
		window.$message?.error("Some updates failed");
	} finally {
		isUpdatingAll.value = false;
	}
};

// 查看更新日志
const viewChangelog = (modId: string) => {
	const mod = modsToUpdate.value.find((m) => m.id === modId);
	if (mod) {
		window.$message?.info(`Changelog for ${mod.name}:
- Improved performance
- Fixed bugs
- Added new features`);
	}
};
</script>

<style scoped>
.update-mod-card {
  height: 100%;
}

.update-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid #eee;
}
</style>