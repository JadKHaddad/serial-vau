// Utilities
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
