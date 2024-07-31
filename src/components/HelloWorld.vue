<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height mx-auto" max-width="900">
      <v-img class="mb-4" height="150" src="@/assets/logo.png" />
      <v-table height="300px" fixed-header>
        <thead>
          <tr>
            <th class="text-left">
              Name
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in serialPorts" :key="item.name">
            <td>{{ item.name }}</td>
          </tr>
        </tbody>
      </v-table>
      <v-btn @click="refreshSerialPorts">
        Refresh
      </v-btn>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from '@tauri-apps/api/event'

type SerialPort = {
  name: string
}

const serialPorts = ref<SerialPort[]>([]);

let unlistenSerialPortsEvent: UnlistenFn;

onMounted(async () => {
  getSerialPorts();

  unlistenSerialPortsEvent = await listen('serial_ports_event', (event) => {
    serialPorts.value = event.payload as SerialPort[];
  });
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

const getSerialPorts = () => {
  invoke('get_serial_ports')
    .then((response) => {
      serialPorts.value = response as SerialPort[];
    })
    .catch((error) => {
      console.error(error);
    });
};


</script>
