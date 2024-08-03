<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height mx-auto" max-width="900">
      <v-img class="mb-4" height="150" src="@/assets/logo.png" />
      <v-list class="mb-4">
        <v-list-item v-for="(port, index) in managedSerialPorts" :key="port.name">
          <v-row class="mb-4" align="center">
            <v-col cols="auto">
              <v-list-item-title>{{ port.name }}</v-list-item-title>
            </v-col>
            <v-icon :color="port.status === Status.Open ? 'green' : 'red'" :size="16">
              {{ port.status === Status.Open ? 'mdi-check-circle' : 'mdi-close-circle' }}
            </v-icon>
            <v-icon v-if="port.read_state" :color="port.read_state === ReadState.Read ? 'green' : 'red'" :size="16"
              class="ml-2">
              {{ port.read_state === ReadState.Read ? 'mdi-play-circle-outline' : 'mdi-stop-circle-outline' }}
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
            <v-chip class="mb-4" v-for="(subscribed, subToIndex) in port.subscribed_to" :key="subToIndex"
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
                <v-list-item v-for="( managedPort, _) in managedSerialPorts" :key="managedPort.name"
                  @click="subscribe(managedPort.name, port.name)">
                  <v-list-item-title>{{ managedPort.name }}</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>

            <v-btn @click="openSerialPort({ name: port.name, initial_read_state: ReadState.Read })" variant="plain">
              Open
            </v-btn>
            <v-btn @click="closeSerialPort(port.name)" variant="plain">
              Close
            </v-btn>
            <v-btn @click="toggleReadState(port.name)" variant="plain">
              Toggle Read
            </v-btn>
          </v-list-item-action>

          <v-text-field v-if="port.status === Status.Open" v-model="portValues[index]" label="Send value"
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

      <v-btn @click="refreshSerialPorts" class="ml-4">
        Refresh
      </v-btn>
      <v-btn @click="doError" class="ml-4" color="error">
        Error
      </v-btn>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

enum Status {
  Closed = "Closed",
  Open = "Open",
}

enum ReadState {
  Read = "Read",
  Stop = "Stop",
}

interface ManagedSerialPort {
  name: string;
  status: Status;
  subscriptions: string[];
  subscribed_to: string[];
  read_state?: ReadState;
}

interface OpenSerialPortOptions {
  name: string;
  initial_read_state: ReadState;
}

const managedSerialPorts = ref<ManagedSerialPort[]>([]);
const portValues = ref<string[]>([]);
const broadcastValue = ref<string>('');

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

const clearBroadcastValue = () => {
  broadcastValue.value = '';
};

const clearSerialPortValue = (name: string) => {
  portValues.value = portValues.value.map((_, index) => index === managedSerialPorts.value.findIndex((port) => port.name === name) ? '' : portValues.value[index]);
}

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
  invoke('subscribe', { from, to })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
};

const unsubscribe = (from: string, to: string) => {
  invoke('unsubscribe', { from, to })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
};

const toggleReadState = (name: string) => {
  invoke('toggle_read_state', { name })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
};


</script>
