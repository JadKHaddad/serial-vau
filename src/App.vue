<template>
  <v-app>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted } from "vue";
import { useListenerStore } from "@/stores/listenerStore";

const { setupListeners, cleanupListeners } = useListenerStore();

onMounted(async () => {
  setupListeners();
});

onUnmounted(() => {
  cleanupListeners();
});
</script>

<style>
/* Remove the scrollbar */
html {
  overflow-y: auto;
}
</style>
