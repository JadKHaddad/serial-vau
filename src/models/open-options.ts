import { ReadState } from "./managed-serial-port";

export enum DataBits {
    Five = "five",
    Six = "six",
    Seven = "seven",
    Eight = "eight" // default
}

export enum FlowControl {
    None = "none", // default
    Software = "software",
    Hardware = "hardware"
}

export enum Parity {
    None = "none", // default
    Odd = "odd",
    Even = "even"
}

export enum StopBits {
    One = "one", // default
    Two = "two"
}

export type Duration = {
    secs: number;
    nanos: number;
}

export type OpenSerialPortOptions = {
    name: string;
    initialReadState: ReadState;
    baudRate: number;
    dataBits: DataBits;
    flowControl: FlowControl;
    parity: Parity;
    stopBits: StopBits;
    timeout: Duration; // default: 0
}