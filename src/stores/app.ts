// Utilities
import { PacketData } from '@/models/intern';
import { ManagedSerialPort } from '@/models/models';
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const managedSerialPorts = ref<ManagedSerialPort[]>([]);
  const packetDictionary = ref<Record<string, PacketData[]>>({});

  function addPacket(name: string, data: PacketData) {
    if (!packetDictionary.value[name]) {
      packetDictionary.value[name] = [];
    }
    packetDictionary.value[name].push(data);
  }

  return { managedSerialPorts, packetDictionary, addPacket }
})
