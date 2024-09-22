<template v-if="packetDisplay(packet)">
  <v-row>
    <v-col>
      {{ dateFormat(packet.time) }}
      <span :style="{ color: packetTypeColor }">
        {{ packet.type_value }} </span
      >:
      <span>
        {{ packet.message }}
      </span>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
export type PacketDataType = {
  time: string | Date;
  message: string;
  type_value: string;
  from?: string | Date;
};

enum PacketOriginColor {
  direct = "green",
  broadcast = "brown",
  subscription = "green",
  icoming = "yellow",
}

const props = defineProps<{ packet: PacketDataType }>();

const dateFormat = (date: string | Date): string => {
  return `${new Date(date).toLocaleDateString()}, ${new Date(
    date
  ).toLocaleTimeString()}`;
};

const packetTypeColor = computed<string>(() => {
  return PacketOriginColor[
    props.packet.type_value.toLocaleLowerCase() as keyof typeof PacketOriginColor
  ];
});
</script>
