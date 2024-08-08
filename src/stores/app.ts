// TODO: make every function to the backend (tauri or any other) that required user interaction return a value and set it here in the sate.
// Events should only be processed, when something in the backend happened without user interaction.
// TODO: Move the functions and the event listeners to a single module. move the backend specific functions and events to a backend specific module
// then use the backend specific module in the Functions/Events module.
// this will allow us to handle seperate backends (taurim or web) (web events are socketio events).
import { PacketData } from '@/models/intern/packet-data';
import { ManagedSerialPort } from '@/models/managed-serial-port';
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const managedSerialPorts = ref<ManagedSerialPort[]>([]);
  const packets = ref<Record<string, PacketData[]>>({});

  /**
   * Adds a packet to the corresponding port.
   * If the port does not exist, it will be created.
   */
  function addPacket(portName: string, data: PacketData) {
    if (!packets.value[portName]) {
      packets.value[portName] = [];
    }
    packets.value[portName].push(data);
  }

  return { managedSerialPorts, packets, addPacket }
})
