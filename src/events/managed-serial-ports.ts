import { ManagedSerialPort } from "@/models/managed-serial-port";


export interface ManagedSerialPortsEvent {
    ports: ManagedSerialPort[];
}