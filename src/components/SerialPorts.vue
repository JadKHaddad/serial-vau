<template>
    <v-container>
        <v-list class="mb-4">
            <v-list-item v-for="(port, index) in app.managedSerialPorts" :key="port.name">
                <v-row class="mb-4" align="center">
                    <v-col cols="auto">
                        <v-list-item-title>{{ port.name }}</v-list-item-title>
                    </v-col>
                    <v-icon :color="port.status.type === StatusType.Open ? 'green' : 'red'" :size="16">
                        {{ port.status.type === StatusType.Open ? 'mdi-check-circle' : 'mdi-close-circle' }}
                    </v-icon>
                    <v-icon v-if="port.status.type === StatusType.Open && port.status.content.readState"
                        :color="port.status.content?.readState === ReadState.Read ? 'green' : 'red'" :size="16"
                        class="ml-2">
                        {{ port.readState === ReadState.Read ? 'mdi-play-circle-outline' :
                            'mdi-stop-circle-outline' }}
                    </v-icon>
                </v-row>

                <v-list-item-subtitle class="mb-4">Subscriptions:</v-list-item-subtitle>
                <v-chip-group column>
                    <v-chip class="mb-4" v-for="(subscription, subIndex) in port.subscriptions" :key="subIndex" closable
                        v-on:click:close="unsubscribe(port.name, subscription)">
                        {{ subscription }}
                    </v-chip>
                </v-chip-group>

                <v-list-item-subtitle class="mb-4">Subscribed To:</v-list-item-subtitle>
                <v-chip-group column>
                    <v-chip class="mb-4" v-for="(subscribed, subToIndex) in port.subscribedTo" :key="subToIndex"
                        v-on:click:close="unsubscribe(subscribed, port.name)" closable>
                        {{ subscribed }}
                    </v-chip>
                </v-chip-group>

                <v-list-item-action class="mb-4">
                    <v-menu>
                        <template v-slot:activator="{ props }">
                            <v-btn v-bind="props" variant="plain">
                                Subscribe
                            </v-btn>
                        </template>
                        <v-list>
                            <v-list-item v-for="( managedPort, _) in app.managedSerialPorts" :key="managedPort.name"
                                @click="subscribe(managedPort.name, port.name)">
                                <v-list-item-title>{{ managedPort.name }}</v-list-item-title>
                            </v-list-item>
                        </v-list>
                    </v-menu>

                    <v-btn @click="openSerialPort({ name: port.name, initialReadState: ReadState.Read })"
                        variant="plain">
                        Open
                    </v-btn>
                    <v-btn @click="closeSerialPort(port.name)" variant="plain">
                        Close
                    </v-btn>
                    <v-btn @click="toggleReadState(port.name)" variant="plain">
                        Toggle Read
                    </v-btn>
                </v-list-item-action>

                <v-text-field v-if="port.status.type === StatusType.Open" v-model="portValues[index]" label="Send value"
                    :append-icon="portValues[index] ? 'mdi-send' : ''"
                    @click:append="sendToSerialPortAncClearValue(port.name, portValues[index])" clearable
                    @click:clear="clearSerialPortValue(port.name)"></v-text-field>

                <v-divider class="mb-4 mt-4"></v-divider>
            </v-list-item>
        </v-list>

        <v-text-field v-model="broadcastValue" label="Enter value to send to all ports"
            :append-icon="broadcastValue ? 'mdi-send' : ''"
            @click:append="sendToAllSerialPortsAndClearBroadcastValue(broadcastValue)" clearable
            @click:clear="clearBroadcastValue"></v-text-field>
    </v-container>
</template>

<script setup lang="ts">
// TODO: Create SerialModel.vue component to be reused. here and in serial monitor
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { StatusType, ReadState } from '@/models/managed-serial-port';
import { useAppStore } from '@/stores/app';
import { OpenSerialPortOptions } from '@/models/open-options';


const app = useAppStore()

const portValues = ref<string[]>([]);
const broadcastValue = ref<string>('');

const clearBroadcastValue = () => {
    broadcastValue.value = '';
};

const clearSerialPortValue = (name: string) => {
    portValues.value = portValues.value.map((_, index) => index === app.managedSerialPorts.findIndex((port) => port.name === name) ? '' : portValues.value[index]);
}

const openSerialPort = (options: OpenSerialPortOptions) => {
    app.openSerialPort(options);
}

const closeSerialPort = (name: string) => {
    app.closeSerialPort(name);
}

const sendToSerialPort = (name: string, value: string) => {
    invoke('send_to_serial_port', { name, value })
        .then((response) => {

        })
        .catch((error) => {
            console.error(error);
        });
};

const sendToSerialPortAncClearValue = (name: string, value: string) => {
    sendToSerialPort(name, value);
    clearSerialPortValue(name);
};

const sendToAllSerialPorts = (value: string) => {
    invoke('send_to_all_serial_ports', { value })
        .then((response) => {

        })
        .catch((error) => {
            console.error(error);
        });
};

const sendToAllSerialPortsAndClearBroadcastValue = (value: string) => {
    sendToAllSerialPorts(value);
    clearBroadcastValue();
};

const subscribe = (from: string, to: string) => {
    app.subscribe(from, to);
};

const unsubscribe = (from: string, to: string) => {
    app.unsubscribe(from, to);
};

const toggleReadState = (name: string) => {
    app.toggleReadState(name);
};
</script>