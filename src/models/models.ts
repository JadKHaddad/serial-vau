export enum Status {
    Closed = "Closed",
    Open = "Open",
}

export enum ReadState {
    Read = "Read",
    Stop = "Stop",
}

export interface ManagedSerialPort {
    name: string;
    status: Status;
    subscriptions: string[];
    subscribed_to: string[];
    read_state?: ReadState;
}

export interface OpenSerialPortOptions {
    name: string;
    initial_read_state: ReadState;
}
