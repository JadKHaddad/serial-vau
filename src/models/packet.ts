export type IncomingPacket = {
    line: string;
}

export type SubscriptionPacketOrigin = {
    name: string;
}

export enum PacketOriginType {
    Direct = "direct",
    Broadcast = "broadcast",
    Subscription = "subscription",
}

export type PacketOrigin =
    | { type: PacketOriginType.Direct }
    | { type: PacketOriginType.Broadcast }
    | { type: PacketOriginType.Subscription; content: SubscriptionPacketOrigin };


export type OutgoingPacket = {
    packetOrigin: PacketOrigin;
    value: string;
}

export enum PacketDirectionType {
    Incoming = "incoming",
    Outgoing = "outgoing",
}

export type PacketDirection =
    | { type: PacketDirectionType.Incoming; content: IncomingPacket }
    | { type: PacketDirectionType.Outgoing; content: OutgoingPacket };

export type Packet = {
    packetDirection: PacketDirection;
    portName: string;
    timestampMillis: number;
}