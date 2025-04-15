<template>
  <div class="main-content">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
// Component receives content through slots
</script>

<style scoped>
.main-content {
  height: 100%;
  width: 100%;
  background: rgb(0, 0, 0, 0.25) !important;
  border-radius: 1rem 1rem 0 0;
}
</style>