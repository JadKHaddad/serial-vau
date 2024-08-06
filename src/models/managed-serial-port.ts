export type Status =
    | { type: StatusType.Closed; content: null }
    | { type: StatusType.Open; content: OpenStatus };

export interface OpenStatus {
    readState: ReadState;
}

export enum StatusType {
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

export interface IncomigPacket {
    from: string,
    line: string,
    timestampMillis: number
}
