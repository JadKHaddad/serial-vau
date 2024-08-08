// intern models that are used by the app. Not for communication

import { PacketDirection } from "../packet";

/**
 * Has the same fields as models/Packet except the name.
 * This will be stored in a list as a value of map.
 * The key for the map is the port name.
 */
export interface PacketData {
    packetDirection: PacketDirection;
    timestampMillis: number;
}