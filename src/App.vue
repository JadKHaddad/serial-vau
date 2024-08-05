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
import { ManagedSerialPorts } from './events/managed-serial-ports';

const theme = useTheme()
const app = useAppStore()

let unlistenThemeChangedEvent: UnlistenFn;
let unlistenSerialPortsEvent: UnlistenFn;
let unlistenSerialLineEvent: UnlistenFn;

onMounted(async () => {
  unlistenThemeChangedEvent = await listen('tauri://theme-changed', (event) => {
    const themeName = event.payload as string;
    if (themeName === 'dark' || themeName === 'light') {
      theme.global.name.value = themeName;
    }
  });

  unlistenSerialPortsEvent = await listen('serial_ports_event', (event) => {
    const managedSerialPortsEvent = event.payload as ManagedSerialPorts;
    app.managedSerialPorts = managedSerialPortsEvent.ports;
  });

  unlistenSerialLineEvent = await listen('serial_line_event', (event) => {
    console.log(event.payload);
  });

  refreshSerialPorts();
});

onUnmounted(() => {
  if (unlistenThemeChangedEvent) {
    unlistenThemeChangedEvent();
  }

  if (unlistenSerialPortsEvent) {
    unlistenSerialPortsEvent();
  }

  if (unlistenSerialLineEvent) {
    unlistenSerialLineEvent();
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