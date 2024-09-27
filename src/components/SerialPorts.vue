<template>
    <v-container class="mt-2">
      <h1 class="mb-1">
        Serial Port
        <v-btn @click="getSerialPorts()" variant="outlined">
            <v-icon>mdi-refresh</v-icon>
        </v-btn>
      </h1>
      <v-list class="mb-4">
          <v-list-item v-for="(port, _) in app.managedSerialPorts" :key="port.name">
              <SerialPort :port="port"></SerialPort>
          </v-list-item>
      </v-list>

      <v-text-field v-model="broadcastValue" label="Enter value to send to all ports"
          :append-icon="broadcastValue ? 'mdi-send' : ''"
          @keydown.enter.prevent="sendToAllSerialPortsAndClearBroadcastValue(broadcastValue)"
          @click:append="sendToAllSerialPortsAndClearBroadcastValue(broadcastValue)" clearable
          @click:clear="clearBroadcastValue">
      </v-text-field>
</v-container>
</template>

<script setup lang="ts">

import { ref } from 'vue';
import { useAppStore } from '@/stores/app';

const app = useAppStore()

const {getSerialPorts} = app

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