import { ReadState } from "./managed-serial-port";

export interface OpenSerialPortOptions {
    name: string;
    initialReadState: ReadState;
}