<template>
    <v-container>
        <v-tabs v-model="selectedPortIndex" background-color="primary" dark>
            <v-tab v-for="portName in portNames" :key="portName">
                {{ portName }}
            </v-tab>
        </v-tabs>
        <v-tabs-window v-model="selectedPortIndex">
            <v-tabs-window-item v-for="portName in portNames" :key="portName" value="portName">
                <v-card class="d-flex flex-column" style="height: 85vh;"> <!-- FIXME: I don't like this -->
                    <v-card-text class="flex-grow-1 overflow-y-auto">
                        <v-list v-if="app.packets[portName]?.length">
                            <v-list-item v-for="(packet, index) in app.packets[portName]" :key="index">
                                <v-list-item-title>{{ packetTitle(packet) }}</v-list-item-title>
                                <v-list-item-subtitle>{{ new Date(packet.timestampMillis).toLocaleString()
                                    }}</v-list-item-subtitle>
                                <v-divider class="mb-1 mt-3"></v-divider>
                            </v-list-item>
                        </v-list>
                    </v-card-text>

                    <v-card-actions v-if="selectedPort?.status.type === StatusType.Open">
                        <v-text-field v-model="portValues[selectedPort.name]" label="Send value"
                            :append-icon="portValues[selectedPort.name] ? 'mdi-send' : ''"
                            @click:append="sendToSerialPortAndClearValue(selectedPort.name, portValues[selectedPort.name])"
                            clearable @click:clear="clearSerialPortValue(selectedPort.name)"></v-text-field>
                    </v-card-actions>
                </v-card>
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

const portNames = computed(() => Object.keys(app.packets));

const selectedPort = computed(() => {
    const selectedPortName = portNames.value[selectedPortIndex.value]
    const selectedPort = app.managedSerialPorts.find(port => port.name === selectedPortName) || null;

    return selectedPort
});

const clearSerialPortValue = (name: string) => {
    portValues.value[name] = ""
};

const packetTitle = (packet: PacketData) => {
    if (packet.packetDirection.type === PacketDirectionType.Outgoing) {
        const origin = packet.packetDirection.content.packetOrigin;

        if (origin.type === PacketOriginType.Direct) {
            return `Direct <| ${packet.packetDirection.content.value}`;
        }

        if (origin.type === PacketOriginType.Broadcast) {
            return `Broadcast <| ${packet.packetDirection.content.value}`;
        }

        if (origin.type === PacketOriginType.Subscription) {
            const from = origin.content.name;
            return `Subscription(${from}) <| ${packet.packetDirection.content.value}`;
        }
    }

    if (packet.packetDirection.type === PacketDirectionType.Incoming) {
        return `|> ${packet.packetDirection.content.line}`;
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
