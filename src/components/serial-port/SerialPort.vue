<template>
    <v-container>
        <v-row align="center">
            <v-col cols="auto">
                <p class="text-subtitle-1">
                    {{ port.name }}
                </p>
            </v-col>
            <v-col>
                <v-icon :color="port.status.type === StatusType.Open ? 'green' : 'red'" :size="16">
                    {{ port.status.type === StatusType.Open ? 'mdi-check-circle' : 'mdi-close-circle' }}
                </v-icon>
                <v-icon v-if="port.status.type === StatusType.Open && port.status.content.readState"
                    :color="port.status.content?.readState === ReadState.Read ? 'green' : 'red'" :size="16"
                    class="ml-2">
                    {{ port.status.content.readState === ReadState.Read ? 'mdi-play-circle-outline' :
                        'mdi-stop-circle-outline' }}
                </v-icon>
            </v-col>
        </v-row>
        <v-row v-if="port.subscriptions.length > 0">
            <v-col cols="auto">
                <p class="text-subtitle-1">
                    Subscriptions
                </p>
            </v-col>
            <v-col>
                <v-chip class="mr-4" v-for="(subscription, subIndex) in port.subscriptions" :key="subIndex" closable
                    size="small" v-on:click:close="unsubscribe(port.name, subscription)">
                    {{ subscription }}
                </v-chip>
            </v-col>
        </v-row>
        <v-row v-if="port.subscribedTo.length > 0">
            <v-col cols="auto">
                <p class="text-subtitle-1">
                    Subscribed To
                </p>
            </v-col>
            <v-col>
                <v-chip class="mr-4" v-for="(subscribed, subToIndex) in port.subscribedTo" :key="subToIndex" closable
                    size="small" v-on:click:close="unsubscribe(subscribed, port.name)">
                    {{ subscribed }}
                </v-chip>
            </v-col>
        </v-row>

        <v-row class="mb-4">
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

            <v-btn @click="openSerialPort({
                name: port.name,
                initialReadState: ReadState.Read,
                baudRate: 115200,
                dataBits: DataBits.Eight,
                flowControl: FlowControl.None,
                parity: Parity.None,
                stopBits: StopBits.One,
                timeout: {
                    secs: 0,
                    nanos: 0
                }
            })" variant="plain">
                Open
            </v-btn>
            <v-btn @click="closeSerialPort" variant="plain">
                Close
            </v-btn>
            <v-btn @click="toggleReadState" variant="plain">
                Toggle Read
            </v-btn>
        </v-row>
    </v-container>
</template>

<script lang="ts" setup>
import { StatusType, ReadState } from '@/models/managed-serial-port';
import { useAppStore } from '@/stores/app';
import { OpenSerialPortOptions, DataBits, FlowControl, Parity, StopBits } from '@/models/open-options';
import { ManagedSerialPort } from '@/models/managed-serial-port';

const props = defineProps<{
    port: ManagedSerialPort
}>()

const app = useAppStore()

const openSerialPort = (options: OpenSerialPortOptions) => {
    app.openSerialPort(options);
}

const closeSerialPort = () => {
    app.closeSerialPort(props.port.name);
}

const subscribe = (from: string, to: string) => {
    app.subscribe(from, to);
};

const unsubscribe = (from: string, to: string) => {
    app.unsubscribe(from, to);
};

const toggleReadState = () => {
    app.toggleReadState(props.port.name);
};

</script>