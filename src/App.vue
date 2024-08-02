<template>
  <v-app>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted } from 'vue';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useTheme } from 'vuetify'

const theme = useTheme()

let unlistenThemeChangedEvent: UnlistenFn;

onMounted(async () => {
  unlistenThemeChangedEvent = await listen('tauri://theme-changed', (event) => {
    const themeName = event.payload as string;
    if (themeName === 'dark' || themeName === 'light') {
      theme.global.name.value = themeName;
    }
  });
});

onUnmounted(() => {
  if (unlistenThemeChangedEvent) {
    unlistenThemeChangedEvent();
  }
});
</script>
