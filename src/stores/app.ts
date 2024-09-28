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

  function getSerialPorts() {
    invoke('get_serial_ports')
      .then((response) => {
        const managedSerialPortsResponse = response as ManagedSerialPort[];

        managedSerialPorts.value = managedSerialPortsResponse;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function openSerialPort(name: string, options: OpenSerialPortOptions) {
    invoke('open_serial_port', { name, options })
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

  function subscribe(from: string, to: string) {
    invoke('subscribe', { from, to })
      .then((response) => {
        const managedSerialPortsResponse = response as ManagedSerialPort[];

        managedSerialPorts.value = managedSerialPortsResponse;
      })
      .catch((error) => {
        console.error(error);
      })
  }

  function unsubscribe(from: string, to: string) {
    invoke('unsubscribe', { from, to })
      .then((response) => {
        const managedSerialPortsResponse = response as ManagedSerialPort[];

        managedSerialPorts.value = managedSerialPortsResponse;
      })
      .catch((error) => {
        console.error(error);
      })
  }

  function toggleReadState(name: string) {
    invoke('toggle_read_state', { name })
      .then((response) => {
        const managedSerialPortsResponse = response as ManagedSerialPort[];

        managedSerialPorts.value = managedSerialPortsResponse;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function sendToSerialPort(name: string, value: string) {
    invoke('send_to_serial_port', { name, value })
      .then((response) => {

      })
      .catch((error) => {
        console.error(error);
      });
  }

  function sendToAllSerialPorts(value: string) {
    invoke('send_to_all_serial_ports', { value })
      .then((response) => {

      })
      .catch((error) => {
        console.error(error);
      });
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

  return { managedSerialPorts, packets, getSerialPorts, openSerialPort, closeSerialPort, subscribe, unsubscribe, toggleReadState, sendToSerialPort, sendToAllSerialPorts, addPacket }
})
