// Utilities
import { ManagedSerialPort } from '@/models/models';
import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const managedSerialPorts = ref<ManagedSerialPort[]>([]);

  return { managedSerialPorts }
})
