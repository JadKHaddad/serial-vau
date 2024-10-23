import { storeToRefs } from "pinia";
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";
import { ManagedSerialPortsEvent } from "@/events/managed-serial-ports";
import { PacketEvent } from "@/events/packet";
import { PacketData } from "@/models/intern/packet-data";
import { useAppStore } from "@/stores/app";
import { useTheme } from "vuetify";

export type RemoveListenerFunction = () => void;

export const useListener = (app = useAppStore()) => {
  const { managedSerialPorts } = storeToRefs(app);
  const { addPacket, getSerialPorts } = app;
  const theme = useTheme();

  const themeChangedEventListener = ref<RemoveListenerFunction>();
  const serialPortEventListener = ref<RemoveListenerFunction>();
  const serialPortPacketEventListener = ref<RemoveListenerFunction>();

  const setupListeners = async () => {
    themeChangedEventListener.value = await listen(
      "tauri://theme-changed",
      (event) => {
        const themeName = event.payload as string;
        if (themeName === "dark" || themeName === "light") {
          theme.global.name.value = themeName;
        }
      }
    );

    serialPortEventListener.value = await listen<ManagedSerialPortsEvent>(
      "serial_ports_event",
      (event) => {
        managedSerialPorts.value = event.payload.ports;
      }
    );

    serialPortPacketEventListener.value = await listen<PacketEvent>(
      "serial_packet_event",
      (event) => {
        const packet = event.payload.packet;

        const packetData: PacketData = {
          packetDirection: packet.packetDirection,
          timestampMillis: packet.timestampMillis,
        };

        addPacket(packet.portName, packetData);
      }
    );
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
