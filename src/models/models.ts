export enum Status {
    Closed = "closed",
    Open = "open",
}

export enum ReadState {
    Read = "read",
    Stop = "stop",
}

export interface ManagedSerialPort {
    name: string;
    status: Status;
    subscriptions: string[];
    subscribedTo: string[];
    readState?: ReadState;
}

export interface OpenSerialPortOptions {
    name: string;
    initialReadState: ReadState;
}
