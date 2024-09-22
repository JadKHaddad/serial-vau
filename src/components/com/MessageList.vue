<template>
  <v-list height="55vh" width="100%">
    <v-list-item
      v-for="(packet, index) in packets"
      variant="elevated"
      :key="index"
      class="px-4 my-1"
    >
      <v-list-item-title class="pa-auto">
        <message-list-item :packet="packetData(packet)" />
      </v-list-item-title>
    </v-list-item>
  </v-list>
</template>

<script lang="ts" setup>
import { PacketData } from "@/models/intern/packet-data";
import {type PacketDataType } from "@/components/com/MessageListItem.vue"
import {
  PacketDirectionType,
  PacketOrigin,
  PacketOriginType,
} from "@/models/packet";

defineProps<{ packets: PacketData[] }>();


const getPacketData = (
  time: string | Date,
  message: string,
  type_value: string,
  from?: string | Date
): PacketDataType => {
  const result = {
    time,
    message,
    type_value,
    from,
  };
  return result;
};

const getPacketOriginTypeByString = (type: PacketOriginType): string => {
  if (type === PacketOriginType.Direct) return "Direct";
  if (type === PacketOriginType.Subscription) return "Subscription";
  if (type === PacketOriginType.Broadcast) return "Broadcast";

  return "Incoming";
};

const packetData = (packet: PacketData): PacketDataType => {
  const time: Date = new Date(packet.timestampMillis);
  let from: string | Date | undefined = undefined;

  if (packet.packetDirection.type === PacketDirectionType.Incoming) {
    return getPacketData(time, "", "Incoming");
  }

  const origin: PacketOrigin = packet.packetDirection.content.packetOrigin;
  const message: string = packet.packetDirection.content.value;
  const typeString = getPacketOriginTypeByString(origin.type);

  if (origin.type === PacketOriginType.Subscription) {
    from = origin.content.name;
  }

  return getPacketData(time, message, typeString, from);
};
</script>
