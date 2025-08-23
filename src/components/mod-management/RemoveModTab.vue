<template>
  <n-card :bordered="false" class="remove-mod-card">
    <n-h2>Remove Mods</n-h2>

    <!-- 搜索框 -->
    <n-input
        v-model:value="searchQuery"
        clearable
        placeholder="Search mods to remove..."
        style="margin-bottom: 20px"
    >
      <template #prefix>
        <n-icon>
          <SearchOutline/>
        </n-icon>
      </template>
    </n-input>

    <!-- 无数据状态 -->
    <n-result
        v-if="filteredMods.length === 0"
        description="Try adjusting your search query"
        status="info"
        title="No mods found"
    />

    <!-- Mod列表 -->
    <div v-else>
      <n-list>
        <n-list-item v-for="mod in filteredMods" :key="mod.steamWorkshopId">
          <n-thing>
            <template #avatar>
              <n-checkbox
                  :checked="selectedMods.includes(mod.steamWorkshopId)"
                  @update:checked="toggleModSelection(mod.steamWorkshopId)"
              />
            </template>

            <template #header>
              <n-ellipsis style="max-width: 200px">
                {{ mod.name }}
              </n-ellipsis>
            </template>

            <template #description>
              <n-space size="small" vertical>
                <n-text depth="3">Version: {{ mod.modVersion }}</n-text>
                <n-text depth="3">Steam ID: {{ mod.steamWorkshopId }}</n-text>
              </n-space>
            </template>

            <template #actions>
              <n-button
                  secondary
                  size="small"
                  type="error"
                  @click="removeMod(mod.steamWorkshopId)"
              >
                Remove
              </n-button>
            </template>
          </n-thing>
        </n-list-item>
      </n-list>

      <!-- 批量操作 -->
      <div v-if="selectedMods.length > 0" class="bulk-actions">
        <n-space align="center" justify="space-between">
          <n-text>{{ selectedMods.length }} mod(s) selected</n-text>
          <n-button
              type="error"
              @click="removeSelectedMods"
          >
            Remove Selected
          </n-button>
        </n-space>
      </div>
    </div>
  </n-card>
</template>

<script lang="ts" setup>
import {computed, ref} from 'vue'
import {SearchOutline} from '@vicons/ionicons5'
import {installed_mod, refreshInstalledMods,} from "../../composables/useModManager"

// 响应式数据
const searchQuery = ref("")
const selectedMods = ref<string[]>([])

// 过滤后的mods
const filteredMods = computed(() => {
  if (!searchQuery.value) return installed_mod.value

  const query = searchQuery.value.toLowerCase()
  return installed_mod.value.filter(mod =>
      mod.name.toLowerCase().includes(query) ||
      mod.steamWorkshopId.toLowerCase().includes(query)
  )
})

// 切换mod选择状态
const toggleModSelection = (modId: string) => {
  const index = selectedMods.value.indexOf(modId)
  if (index > -1) {
    selectedMods.value.splice(index, 1)
  } else {
    selectedMods.value.push(modId)
  }
}

// 删除单个mod
const removeMod = async (modId: string) => {
  const mod = installed_mod.value.find(m => m.steamWorkshopId === modId)
  if (!mod) return

  const confirmed = await window.$dialog?.warning({
    title: 'Confirm Removal',
    content: `Are you sure you want to remove "${mod.name}"?`,
    positiveText: 'Remove',
    negativeText: 'Cancel'
  })

  if (confirmed) {
    try {
      // 模拟删除操作
      window.$message?.success(`Removed ${mod.name} successfully!`)
      await refreshInstalledMods()
    } catch (error) {
      window.$message?.error(`Failed to remove ${mod.name}`)
    }
  }
}

// 删除选中的mods
const removeSelectedMods = async () => {
  if (selectedMods.value.length === 0) return

  const confirmed = await window.$dialog?.warning({
    title: 'Confirm Bulk Removal',
    content: `Are you sure you want to remove ${selectedMods.value.length} mod(s)?`,
    positiveText: 'Remove All',
    negativeText: 'Cancel'
  })

  if (confirmed) {
    try {
      // 模拟批量删除操作
      window.$message?.success(`Removed ${selectedMods.value.length} mod(s) successfully!`)
      await refreshInstalledMods()
      selectedMods.value = [] // 清空选择
    } catch (error) {
      window.$message?.error('Failed to remove mods')
    }
  }
}
</script>

<style scoped>
.remove-mod-card {
  height: 100%;
}

.bulk-actions {
  margin-top: 20px;
  padding: 16px;
  border-top: 1px solid #eee;
}
</style>