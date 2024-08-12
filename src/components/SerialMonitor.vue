<template>
    <v-container>
        <v-tabs v-model="selectedPortIndex" background-color="primary" dark>
            <v-tab v-for="portName in portNames" :key="portName">
                <v-row align="center">
                    <v-col cols="auto">
                        <p class="text-subtitle-2">
                            {{ portName }}
                        </p>
                    </v-col>
                    <v-col v-if="findManagedPort(portName)">
                        <v-icon :color="findManagedPort(portName)?.status.type === StatusType.Open ? 'green' : 'red'"
                            :size="16">
                            {{ findManagedPort(portName)?.status.type === StatusType.Open ? 'mdi-check-circle' :
                                'mdi-close-circle'
                            }}
                        </v-icon>
                    </v-col>
                </v-row>
            </v-tab>
        </v-tabs>
        <v-tabs-window v-model="selectedPortIndex">
            <v-tabs-window-item v-for="portName in portNames" :key="portName" value="portName">
                <v-container class="mt-4">
                    <v-row>
                        <SerialPort v-if="selectedPort" :port="selectedPort"></SerialPort>
                    </v-row>

                    <v-row class="flex-column" style="max-height: 70vh;">
                        <!-- FIXME: Scrolling up should freeze the list -->
                        <!-- Currently: when items are appended, all other items are moving up due to the limited number of items to display -->
                        <v-list v-if="limitedPackets(portName)?.length">
                            <v-list-item v-for="(packet, index) in limitedPackets(portName)" :key="index">
                                <!-- <v-list-item-title>{{  }}</v-list-item-title> -->
                                <v-list-item-subtitle>{{ packetDisplay(packet) }}</v-list-item-subtitle>
                            </v-list-item>
                        </v-list>
                    </v-row>

                    <v-row v-if="selectedPort && selectedPort.status.type === StatusType.Open">
                        <v-text-field v-model="portValues[selectedPort.name]" label="Send value"
                            :append-icon="portValues[selectedPort.name] ? 'mdi-send' : ''"
                            @click:append="sendToSerialPortAndClearValue(selectedPort.name, portValues[selectedPort.name])"
                            clearable @click:clear="clearSerialPortValue(selectedPort.name)">
                        </v-text-field>
                    </v-row>

                </v-container>
            </v-tabs-window-item>
        </v-tabs-window>
    </v-container>
</template>


<script lang="ts" setup>
// TODO: auto scroll
import { ref, computed } from 'vue';
import { PacketData } from '@/models/intern/packet-data';
import { PacketDirectionType, PacketOriginType } from '@/models/packet';
import { StatusType } from '@/models/managed-serial-port';
import { useAppStore } from '@/stores/app';

const app = useAppStore();

const selectedPortIndex = ref<number>(0);
const portValues = ref<Record<string, string>>({});
const portDisplayPacketsLimits = ref<Record<string, number>>({});
const portNames = computed(() => Object.keys(app.packets));

const selectedPort = computed(() => {
    const selectedPortName = portNames.value[selectedPortIndex.value]
    const selectedPort = app.managedSerialPorts.find(port => port.name === selectedPortName) || null;

    return selectedPort
});

const findManagedPort = (portName: string) => {
    return app.managedSerialPorts.find(port => port.name === portName) || null;
};

const limitedPackets = (portName: string) => {
    const packetLimit = portDisplayPacketsLimits.value[portName] || 100;

    const data = app.packets[portName]

    return data.slice(Math.max(data.length - packetLimit, 0));
};

const clearSerialPortValue = (name: string) => {
    portValues.value[name] = ""
};

const packetDisplay = (packet: PacketData) => {

    const time = new Date(packet.timestampMillis).toLocaleString();

    if (packet.packetDirection.type === PacketDirectionType.Outgoing) {
        const origin = packet.packetDirection.content.packetOrigin;

        if (origin.type === PacketOriginType.Direct) {
            return `${time}: Direct: ${packet.packetDirection.content.value}`;
        }

        if (origin.type === PacketOriginType.Broadcast) {
            return `${time}: Broadcast: ${packet.packetDirection.content.value}`;
        }

        if (origin.type === PacketOriginType.Subscription) {
            const from = origin.content.name;
            return `${time}: Subscription(${from}): ${packet.packetDirection.content.value}`;
        }
    }

    if (packet.packetDirection.type === PacketDirectionType.Incoming) {
        return `${time}: ${packet.packetDirection.content.line}`;
    }
};

const sendToSerialPort = (name: string, value: string) => {
    app.sendToSerialPort(name, value);
};

const sendToSerialPortAndClearValue = (name: string, value: string) => {
    sendToSerialPort(name, value);
    clearSerialPortValue(name);
};
</script>
