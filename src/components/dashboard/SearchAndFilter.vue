<template>
  <n-card :bordered="false" class="search-filter-card">
    <n-space align="center" justify="space-between">
      <!-- 搜索框 -->
      <n-input
          v-model:value="searchQuery"
          clearable
          placeholder="Search mods..."
          style="width: 300px"
      >
        <template #prefix>
          <n-icon>
            <SearchOutline/>
          </n-icon>
        </template>
      </n-input>

      <!-- 过滤器 -->
      <n-space>
        <n-select
            v-model:value="statusFilter"
            :options="statusOptions"
            style="width: 120px"
        />
        <n-select
            v-model:value="typeFilter"
            :options="typeOptions"
            style="width: 120px"
        />
      </n-space>
    </n-space>
  </n-card>
</template>

<script lang="ts" setup>
import {ref, watch} from "vue";
import {SearchOutline} from "@vicons/ionicons5";
import type {SelectOption} from "naive-ui";

// 响应式数据
const searchQuery = ref("");
const statusFilter = ref("all");
const typeFilter = ref("all");

// 下拉选项
const statusOptions: SelectOption[] = [
  {label: "All Status", value: "all"},
  {label: "Enabled", value: "enabled"},
  {label: "Disabled", value: "disabled"},
];

const typeOptions: SelectOption[] = [
  {label: "All Types", value: "all"},
  {label: "Local", value: "local"},
  {label: "Remote", value: "remote"},
];

// 监听变化并发出事件
const emit =
    defineEmits<
        (
            e: "filter-change",
            filters: {
              searchQuery: string;
              statusFilter: string;
              typeFilter: string;
            },
        ) => void
    >();

watch([searchQuery, statusFilter, typeFilter], () => {
  emit("filter-change", {
    searchQuery: searchQuery.value,
    statusFilter: statusFilter.value,
    typeFilter: typeFilter.value,
  });
});
</script>

