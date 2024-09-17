export type Status =
    | { type: StatusType.Closed }
    | { type: StatusType.Open; content: OpenStatus };

export type OpenStatus = {
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

export type ManagedSerialPort = {
    name: string;
    status: Status;
    subscriptions: string[];
    subscribedTo: string[];
}

export type IncomigPacket = {
    from: string,
    line: string,
    timestampMillis: number
}
