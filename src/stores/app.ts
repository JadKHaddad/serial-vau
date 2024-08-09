// TODO: make every function to the backend (tauri or any other) that required user interaction return a value and set it here in the sate.
// Events should only be processed, when something in the backend happened without user interaction.
// TODO: Move the functions and the event listeners to a single module. move the backend specific functions and events to a backend specific module
// then use the backend specific module in the Functions/Events module.
// this will allow us to handle seperate backends (taurim or web) (web events are socketio events).
import { PacketData } from '@/models/intern/packet-data';
import { ManagedSerialPort } from '@/models/managed-serial-port';
import { OpenSerialPortOptions } from '@/models/open-options';
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api';

export const useAppStore = defineStore('app', () => {
  const managedSerialPorts = ref<ManagedSerialPort[]>([]);
  const packets = ref<Record<string, PacketData[]>>({});

  function openSerialPort(options: OpenSerialPortOptions) {
    invoke('open_serial_port', { options })
      .then((response) => {
        const managedSerialPortsResponse = response as ManagedSerialPort[];

        managedSerialPorts.value = managedSerialPortsResponse;
      })
      .catch((error) => {
        console.error(error);
      })
  }

  function closeSerialPort(name: string) {
    invoke('close_serial_port', { name })
      .then((response) => {
        const managedSerialPortsResponse = response as ManagedSerialPort[];

        managedSerialPorts.value = managedSerialPortsResponse;
      })
      .catch((error) => {
        console.error(error);
      })
  }

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

  return { managedSerialPorts, packets, openSerialPort, closeSerialPort, addPacket }
})
