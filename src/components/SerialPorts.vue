<template>
    <v-container>
        <v-list class="mb-4">
            <v-list-item v-for="(port, _) in app.managedSerialPorts" :key="port.name">
                <SerialPort :port="port"></SerialPort>
            </v-list-item>
        </v-list>

        <v-text-field v-model="broadcastValue" label="Enter value to send to all ports"
            :append-icon="broadcastValue ? 'mdi-send' : ''"
            @click:append="sendToAllSerialPortsAndClearBroadcastValue(broadcastValue)" clearable
            @click:clear="clearBroadcastValue"></v-text-field>
    </v-container>
</template>

<script setup lang="ts">

import { ref } from 'vue';
import { useAppStore } from '@/stores/app';

const app = useAppStore()

const broadcastValue = ref<string>('');

const clearBroadcastValue = () => {
    broadcastValue.value = '';
};

const sendToAllSerialPorts = (value: string) => {
    app.sendToAllSerialPorts(value);
};

const sendToAllSerialPortsAndClearBroadcastValue = (value: string) => {
    sendToAllSerialPorts(value);
    clearBroadcastValue();
};


</script>