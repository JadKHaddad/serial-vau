import { storeToRefs } from "pinia";
import { Event, UnlistenFn } from "@tauri-apps/api/event";
import { ref } from "vue";
import { ManagedSerialPortsEvent } from "@/events";
import { PacketData } from "@/models/intern/packet-data";
import {
  listenErrorEvent,
  listenPacketEvent,
  listenSerialPortEvent,
  listenThemeChangedEvent,
} from "@/api/listener";
import { useAppStore } from "@/stores/app";
import { useTheme } from "vuetify";

export const useListener = (app = useAppStore()) => {
  const { managedSerialPorts } = storeToRefs(app);
  const { addPacket, getSerialPorts } = app;
  const theme = useTheme();

  const themeChangedEventListener = ref<UnlistenFn>();
  const serialPortEventListener = ref<UnlistenFn>();
  const serialPortPacketEventListener = ref<UnlistenFn>();
  const errorEventListener = ref<UnlistenFn>();

  const onThemeEventListenerTrigger = (event: Event<string>) => {
    const themeName = event.payload;
    if (themeName === "dark" || themeName === "light") {
      theme.global.name.value = themeName;
    }
  };

  const onSerialPortPacketEventListener = (event: Event<any>) => {
    const packet = event.payload.packet;

    const packetData: PacketData = {
      packetDirection: packet.packetDirection,
      timestampMillis: packet.timestampMillis,
    };

    addPacket(packet.portName, packetData);
  };

  const onSerialPortEventListener = (event: Event<ManagedSerialPortsEvent>) => {
    managedSerialPorts.value = event.payload.ports;
  };

  const onErrorEventListener = (event: Event<ErrorEvent>) => {
    console.error(event.payload.error);
  };

  const setupListeners = async () => {
    themeChangedEventListener.value = await listenThemeChangedEvent(
      onThemeEventListenerTrigger
    );

    serialPortEventListener.value = await listenSerialPortEvent(
      onSerialPortEventListener
    );

    serialPortPacketEventListener.value = await listenPacketEvent(
      onSerialPortPacketEventListener
    );

    errorEventListener.value = await listenErrorEvent(onErrorEventListener);

    getSerialPorts();
  };

  const cleanupListeners = () => {
    if (themeChangedEventListener.value) {
      themeChangedEventListener.value();
      themeChangedEventListener.value = undefined;
    }
    if (serialPortEventListener.value) {
      serialPortEventListener.value();
      serialPortEventListener.value = undefined;
    }
    if (serialPortPacketEventListener.value) {
      serialPortPacketEventListener.value();
      serialPortPacketEventListener.value = undefined;
    }
  };

  return {
    setupListeners,
    cleanupListeners,
  };
};
