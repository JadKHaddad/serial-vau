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
import { ManagedSerialPortsEvent } from './events/managed-serial-ports';
import { PacketEvent } from './events/packet';
import { PacketData } from './models/intern/packet-data';

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
    const managedSerialPortsEvent = event.payload as ManagedSerialPortsEvent;
    app.managedSerialPorts = managedSerialPortsEvent.ports;
  });

  unlistenSerialLineEvent = await listen('serial_packet_event', (event) => {
    const packetEvent = event.payload as PacketEvent;
    const packet = packetEvent.packet;

    const packetData: PacketData = {
      packetDirection: packet.packetDirection,
      timestampMillis: packet.timestampMillis
    };

    app.addPacket(packet.portName, packetData);
  });

  getSerialPorts();
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

const getSerialPorts = () => {
  app.getSerialPorts();
};

</script>

<style>
/* Remove the scrollbar */
html {
  overflow-y: auto
}
</style>