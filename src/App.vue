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
import { useAppStore } from './stores/app';
import { invoke } from '@tauri-apps/api';
import { ManagedSerialPort } from './models/models';

const theme = useTheme()
const app = useAppStore()

let unlistenThemeChangedEvent: UnlistenFn;
let unlistenSerialPortsEvent: UnlistenFn;

onMounted(async () => {
  unlistenThemeChangedEvent = await listen('tauri://theme-changed', (event) => {
    const themeName = event.payload as string;
    if (themeName === 'dark' || themeName === 'light') {
      theme.global.name.value = themeName;
    }
  });

  unlistenSerialPortsEvent = await listen('serial_ports_event', (event) => {
    app.managedSerialPorts = event.payload as ManagedSerialPort[];
  });

  refreshSerialPorts();
});

onUnmounted(() => {
  if (unlistenThemeChangedEvent) {
    unlistenThemeChangedEvent();
  }
});

const refreshSerialPorts = () => {
  invoke('refresh_serial_ports')
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
};

</script>

<style>
/* Remove the scrollbar */
html {
  overflow-y: auto
}
</style>