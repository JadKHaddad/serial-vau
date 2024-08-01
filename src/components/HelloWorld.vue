<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height mx-auto" max-width="900">
      <v-img class="mb-4" height="150" src="@/assets/logo.png" />
      <v-list>
        <v-list-item-group>
          <v-list-item v-for="port in managedSerialPorts" :key="port.name">
            <v-list-item-content>
              <v-list-item-title>{{ port.name }}</v-list-item-title>
              <v-list-item-subtitle>{{ port.status }}</v-list-item-subtitle>
            </v-list-item-content>
            <v-list-item-action>
              <v-btn @click="openSerialPort({ name: port.name })">
                Open
              </v-btn>
            </v-list-item-action>
            <v-list-item-action>
              <v-btn @click="closeSerialPort(port.name)">
                Close
              </v-btn>
            </v-list-item-action>
          </v-list-item>
        </v-list-item-group>
      </v-list>
      <v-btn @click="refreshSerialPorts">
        Refresh
      </v-btn>
      <v-btn @click="doError">
        Error
      </v-btn>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from '@tauri-apps/api/event'

enum Status {
  Closed = "Closed",
  Open = "Open",
}

interface ManagedSerialPort {
  name: string;
  status: Status;
}

interface OpenSerialPortOptions {
  name: string
}

const managedSerialPorts = ref<ManagedSerialPort[]>([]);

let unlistenSerialPortsEvent: UnlistenFn;

onMounted(async () => {
  unlistenSerialPortsEvent = await listen('serial_ports_event', (event) => {
    managedSerialPorts.value = event.payload as ManagedSerialPort[];
  });

  refreshSerialPorts();
});

onUnmounted(() => {
  if (unlistenSerialPortsEvent) {
    unlistenSerialPortsEvent();
  }
});

const refreshSerialPorts = () => {
  invoke('refresh_serial_ports')
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
};

const doError = () => {
  invoke('do_error')
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
}

const openSerialPort = (options: OpenSerialPortOptions) => {
  invoke('open_serial_port', { options })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
}

const closeSerialPort = (name: string) => {
  invoke('close_serial_port', { name })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
}

</script>
