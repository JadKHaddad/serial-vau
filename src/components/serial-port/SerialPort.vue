<template>
    <v-card>
        <v-row class="mb-4" align="center">
            <v-col cols="auto">
                <v-card-title>{{ port.name }}</v-card-title>
            </v-col>
            <v-icon :color="port.status.type === StatusType.Open ? 'green' : 'red'" :size="16">
                {{ port.status.type === StatusType.Open ? 'mdi-check-circle' : 'mdi-close-circle' }}
            </v-icon>
            <v-icon v-if="port.status.type === StatusType.Open && port.status.content.readState"
                :color="port.status.content?.readState === ReadState.Read ? 'green' : 'red'" :size="16" class="ml-2">
                {{ port.readState === ReadState.Read ? 'mdi-play-circle-outline' :
                    'mdi-stop-circle-outline' }}
            </v-icon>
        </v-row>

        <v-card-subtitle v-if="port.subscriptions.length > 0" class="mb-4">Subscriptions</v-card-subtitle>
        <v-chip-group>
            <v-chip class="mb-4" v-for="(subscription, subIndex) in port.subscriptions" :key="subIndex" closable
                v-on:click:close="unsubscribe(port.name, subscription)">
                {{ subscription }}
            </v-chip>
        </v-chip-group>

        <v-card-subtitle v-if="port.subscribedTo.length > 0" class="mb-4">Subscribed To</v-card-subtitle>
        <v-chip-group>
            <v-chip class="mb-4" v-for="(subscribed, subToIndex) in port.subscribedTo" :key="subToIndex"
                v-on:click:close="unsubscribe(subscribed, port.name)" closable>
                {{ subscribed }}
            </v-chip>
        </v-chip-group>

        <v-card-actions class="mb-4">
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
            <!-- TODO: display options -->
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
        </v-card-actions>
    </v-card>
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