import { ManagedSerialPort } from "@/models/managed-serial-port";
import { OpenSerialPortOptions } from "@/models/open-options";
import { invoke } from "@tauri-apps/api";

export enum SerialVauApi {
  GET_SERIAL_PORTS = "get_serial_ports",
  OPEN_SERIAL_PORT = "open_serial_port",
  CLOSE_SERIAL_PORT = "close_serial_port",
  SEND_TO_SERIAL_PORTS = "send_to_all_serial_ports",
  SEND_TO_SERIAL_PORT = "send_to_all_serial_port",
  TOGGLE_READ_STATE = "toggle_read_state",
  SUBSCRIBE = "subscribe",
  UNSUBSCRIBE = "unsubscribe",
}

export const getSerialPorts = async <T = ManagedSerialPort[]>(): Promise<T> => {
  return await invoke<T>(SerialVauApi.GET_SERIAL_PORTS);
};

export const openSerialPort = async <T = ManagedSerialPort[]>(
  name: string,
  options: OpenSerialPortOptions
): Promise<T> => {
  return await invoke<T>(SerialVauApi.OPEN_SERIAL_PORT, {
    name,
    options,
  });
};

export const closeSerialPort = async <T = ManagedSerialPort[]>(
  name: string
): Promise<T> => {
  return invoke<T>(SerialVauApi.CLOSE_SERIAL_PORT, { name });
};

export const subscribe = async <T = ManagedSerialPort[]>(
  from: string,
  to: string
): Promise<T> => {
  return await invoke<T>(SerialVauApi.SUBSCRIBE, { from, to });
};

export const unsubscribe = async <T = ManagedSerialPort[]>(
  from: string,
  to: string
): Promise<T> => {
  return await invoke<T>(SerialVauApi.UNSUBSCRIBE, { from, to });
};

export const toggleReadState = async <T = ManagedSerialPort[]>(
  name: string
): Promise<T> => {
  return await invoke<T>(SerialVauApi.TOGGLE_READ_STATE, {
    name,
  });
};

export const sendToSerialPort = async <T = void>(
  name: string,
  value: string
): Promise<T> => {
  return await invoke<T>(SerialVauApi.SEND_TO_SERIAL_PORT, { name, value });
};

export const sendToAllSerialPorts = async <T = void>(
  value: string
): Promise<T> => {
  return await invoke<T>(SerialVauApi.SEND_TO_SERIAL_PORTS, { value });
};
