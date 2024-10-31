import { PacketData } from "@/models/intern/packet-data";
import { ManagedSerialPort } from "@/models/managed-serial-port";
import { OpenSerialPortOptions } from "@/models/open-options";
import { defineStore } from "pinia";
import * as api from "@/api/api";

export const useAppStore = defineStore("app", () => {
  const managedSerialPorts = ref<ManagedSerialPort[]>([]);
  const packets = ref<Record<string, PacketData[]>>({});

  function getSerialPorts() {
    api
      .getSerialPorts()
      .then((response) => {
        managedSerialPorts.value = response;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function openSerialPort(name: string, options: OpenSerialPortOptions) {
    api
      .openSerialPort(name, options)
      .then((response) => {
        managedSerialPorts.value = response;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function closeSerialPort(name: string) {
    api
      .closeSerialPort(name)
      .then((response) => {
        managedSerialPorts.value = response;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function subscribe(from: string, to: string) {
    api
      .subscribe(from, to)
      .then((response) => {
        managedSerialPorts.value = response;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function unsubscribe(from: string, to: string) {
    api
      .unsubscribe(from, to)
      .then((response) => {
        managedSerialPorts.value = response;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  function toggleReadState(name: string) {
    api
      .toggleReadState(name)
      .then((response) => {
        managedSerialPorts.value = response;
      })
      .catch((error) => {
        console.error("Error toggling read state:", error);
      });
  }

  function sendToSerialPort(name: string, value: string) {
    api.sendToSerialPort(name, value).catch((error) => {
      console.error(error);
    });
  }

  function sendToAllSerialPorts(value: string) {
    api.sendToAllSerialPorts(value).catch((error) => {
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

  return {
    managedSerialPorts,
    packets,
    getSerialPorts,
    openSerialPort,
    closeSerialPort,
    subscribe,
    unsubscribe,
    toggleReadState,
    sendToSerialPort,
    sendToAllSerialPorts,
    addPacket,
  };
});
