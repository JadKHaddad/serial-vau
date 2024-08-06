export interface IncomingPacket {
    line: string;
}

export interface SubscriptionPacketOrigin {
    name: string;
}

export enum PacketOriginType {
    Direct = "direct",
    Broadcast = "broadcast",
    Subscription = "subscription",
}

export type PacketOrigin =
    | { type: PacketOriginType.Direct, content: null }
    | { type: PacketOriginType.Broadcast, content: null }
    | { type: PacketOriginType.Subscription; content: SubscriptionPacketOrigin };


export interface OutgoingPacket {
    packetOrigin: PacketOrigin;
    data: number[];
}

export enum PacketDirectionType {
    Incoming = "incoming",
    Outgoing = "outgoing",
}

export type PacketDirection =
    | { type: PacketDirectionType.Incoming; content: IncomingPacket }
    | { type: PacketDirectionType.Outgoing; content: OutgoingPacket };

export interface Packet {
    packetDirection: PacketDirection;
    portName: string;
    timestampMillis: number;
}