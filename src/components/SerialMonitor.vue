<template>
  <v-container class="mt-2">
    <h1 class="mb-1">
      Monitor
      <v-btn @click="getSerialPorts()" variant="outlined">
        <v-icon>mdi-refresh</v-icon>
      </v-btn>
    </h1>
    <v-tabs v-model="selectedPortIndex" background-color="primary" dark>
      <v-tab v-for="portName in portNames" :key="portName">
        <v-row align="center">
          <v-col cols="auto">
            <p class="text-subtitle-2">
              {{ portName }}
            </p>
          </v-col>
          <v-col v-if="findManagedPort(portName)">
            <v-icon :color="isPortOpen(portName) ? 'green' : 'red'" :size="16">
              {{
                isPortOpen(portName) ? "mdi-check-circle" : "mdi-close-circle"
              }}
            </v-icon>
          </v-col>
        </v-row>
      </v-tab>
    </v-tabs>
    <v-tabs-window v-model="selectedPortIndex">
      <v-tabs-window-item
        v-for="portName in portNames"
        :key="portName"
        value="portName"
      >
        <v-container class="mt-4">
          <v-row>
            <SerialPort v-if="selectedPort" :port="selectedPort"></SerialPort>
          </v-row>
          <v-row>
            <MessageList :packets="limitedPackets(portName)" />
          </v-row>

          <v-row
            v-if="selectedPort && selectedPort.status.type === StatusType.Open"
          >
            <v-text-field
              v-model="portValues[selectedPort.name]"
              class="mt-4"
              color="secondary"
              text-color="primary"
              variant="outlined"
              label="Send value as Direct"
              :append-icon="portValues[selectedPort.name] ? 'mdi-send' : ''"
              @keydown.enter.prevent="
                sendToSerialPortAndClearValue(
                  selectedPort.name,
                  portValues[selectedPort.name]
                )
              "
              @click:append="
                sendToSerialPortAndClearValue(
                  selectedPort.name,
                  portValues[selectedPort.name]
                )
              "
              clearable
              @click:clear="clearSerialPortValue(selectedPort.name)"
            >
            </v-text-field>
          </v-row>
        </v-container>
      </v-tabs-window-item>
    </v-tabs-window>
  </v-container>
</template>

<script lang="ts" setup>
// TODO: auto scroll
import { ref, computed } from "vue";
import { StatusType } from "@/models/managed-serial-port";
import { useAppStore } from "@/stores/app";

const app = useAppStore();
const { getSerialPorts } = app;

const selectedPortIndex = ref<number>(0);
const portValues = ref<Record<string, string>>({});
const portDisplayPacketsLimits = ref<Record<string, number>>({});
const portNames = computed(() => Object.keys(app.packets));

const selectedPort = computed(() => {
  const selectedPortName = portNames.value[selectedPortIndex.value];
  const selectedPort =
    app.managedSerialPorts.find((port) => port.name === selectedPortName) ||
    null;

  return selectedPort;
});

const findManagedPort = (portName: string) => {
  return app.managedSerialPorts.find((port) => port.name === portName) || null;
};

const isPortOpen = (portName: string): boolean => {
  return findManagedPort(portName)?.status.type === StatusType.Open;
};

const limitedPackets = (portName: string) => {
  const packetLimit = portDisplayPacketsLimits.value[portName] || 100;

  const data = app.packets[portName];

  return data.slice(Math.max(data.length - packetLimit, 0));
};

const clearSerialPortValue = (name: string) => {
  portValues.value[name] = "";
};

const sendToSerialPort = (name: string, value: string) => {
  if (value.length == 0) return;

  app.sendToSerialPort(name, value);
};

const sendToSerialPortAndClearValue = (name: string, value: string) => {
  sendToSerialPort(name, value);
  clearSerialPortValue(name);
};
</script>
