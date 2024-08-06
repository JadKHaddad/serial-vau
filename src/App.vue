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
import { ManagedSerialPortsEvent } from './events/managed-serial-ports';
import { PacketEvent } from './events/packet';
import { PacketDirectionType, PacketOriginType } from './models/packet';

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

    console.log(packet);

    if (packet.packetDirection.type === PacketDirectionType.Outgoing) {
      console.log('Is outgoing packet');
      const origin = packet.packetDirection.content.packetOrigin;
      console.log('Origin: ' + origin.type);
      if (origin.type == PacketOriginType.Direct) {
        console.log('Outgoing direct packet');
      }

      else if (origin.type == PacketOriginType.Broadcast) {
        console.log('Outgoing broadcast packet');
      }

      else if (origin.type == PacketOriginType.Subscription) {
        const from = origin.content.name;

        console.log('Outgoing subscription packet from: ' + from);
      }
      else {
        console.log('Unknown outgoing packet origin');
      }

    } else {
      console.log('Is not outgoing packet: ' + packet.packetDirection.type);
    }

    // const packetData: PacketData = {
    //   line: incomingPacket.line,
    //   timestampMillis: incomingPacket.timestampMillis
    // };

    // app.addPacket(incomingPacket.from, packetData);
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