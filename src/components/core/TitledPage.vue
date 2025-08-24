<template>
  <n-h1>
    <slot name="title"></slot>
  </n-h1>
  <n-flex :size="24" vertical>
    <slot></slot>
  </n-flex>
</template>
