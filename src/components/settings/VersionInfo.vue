<template>
  <n-card :bordered="false" class="version-info-card">
    <n-grid :cols="3" :x-gap="16" :y-gap="16" class="animate-fade-in">
      <!-- Game Mod Manager Card -->
      <n-gi>
        <n-card class="animate-slide-in-right" hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <GameController/>
              </n-icon>
              <n-text>{{ $t('version.title') }}</n-text>
            </n-space>
          </template>
          <n-descriptions :column="1" label-placement="left" size="small">
            <n-descriptions-item :label="$t('version.version')">
              <n-text v-text="build_info.version"></n-text>
            </n-descriptions-item>
            <n-descriptions-item :label="$t('version.commit')">
              <n-text v-text="build_info.commit"></n-text>
            </n-descriptions-item>
            <n-descriptions-item :label="$t('version.release_date')">
              <n-text v-text="build_info.date"></n-text>
            </n-descriptions-item>
          </n-descriptions>
        </n-card>
      </n-gi>

      <!-- About Card -->
      <n-gi>
        <n-card class="animate-slide-in-right" hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <InformationCircle/>
              </n-icon>
              <n-text>{{ $t('version.about') }}</n-text>
            </n-space>
          </template>
          <n-space vertical>
            <n-text>{{ $t('version.about_text_1') }}</n-text>
            <n-text>{{ $t('version.about_text_2') }}</n-text>
          </n-space>
        </n-card>
      </n-gi>

      <!-- Links & Support Card -->
      <n-gi>
        <n-card class="animate-slide-in-right" hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <Link/>
              </n-icon>
              <n-text>{{ $t('version.links') }}</n-text>
            </n-space>
          </template>
          <n-space vertical>
            <n-button href="https://github.com/Whth/BaroBaro" tag="a" text type="primary">
              {{ $t('version.repository') }}
            </n-button>
            <n-button href="https://github.com/Whth/BaroBaro/blob/master/README.md" tag="a" text type="primary">
              {{ $t('version.documentation') }}
            </n-button>
            <n-button href="https://github.com/Whth/BaroBaro/discussions" tag="a" text type="primary">
              {{ $t('version.support') }}
            </n-button>
            <n-button href="https://github.com/Whth/BaroBaro/issues" tag="a" text type="primary">
              {{ $t('version.report_issues') }}
            </n-button>
          </n-space>
        </n-card>
      </n-gi>

      <!-- Copyright Card -->
      <n-gi>
        <n-card class="animate-slide-in-right" hoverable size="small">
          <template #header>
            <n-space align="center">
              <n-icon size="20">
                <DocumentText/>
              </n-icon>
              <n-text>{{ $t('version.copyright') }}</n-text>
            </n-space>
          </template>
          <n-space vertical>
            <n-text>{{ $t('version.copyright_text') }}</n-text>
            <n-text>{{ $t('version.license') }}</n-text>
            <n-text depth="3">{{ $t('version.disclaimer') }}</n-text>
          </n-space>
        </n-card>
      </n-gi>
    </n-grid>
  </n-card>
</template>

<script lang="ts" setup>
import {DocumentText, GameController, InformationCircle, Link,} from "@vicons/ionicons5";
import {get_version} from "@/invokes.ts";
import {onMounted, type Ref, ref} from "vue";
import {BuildInfo} from "../../proto/build_info.ts";

const build_info: Ref<BuildInfo> = ref(BuildInfo.create());

onMounted(async () => {
  build_info.value = await get_version();
});
</script>

