// Utilities
import { ManagedSerialPort } from '@/models/managed-serial-port';
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const managedSerialPorts = ref<ManagedSerialPort[]>([]);
  // const packetDictionary = ref<Record<string, PacketData[]>>({});

  // function addPacket(name: string, data: PacketData) {
  //   if (!packetDictionary.value[name]) {
  //     packetDictionary.value[name] = [];
  //   }
  //   packetDictionary.value[name].push(data);
  // }

  return { managedSerialPorts }
})
