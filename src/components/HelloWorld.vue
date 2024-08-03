<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height mx-auto" max-width="900">
      <v-img class="mb-4" height="150" src="@/assets/logo.png" />
      <v-list class="mb-4">
        <v-list-item v-for="(port, index) in managedSerialPorts" :key="port.name">
          <v-list-item-title class="mb-4">{{ port.name }}</v-list-item-title>
          <v-list-item-subtitle class="mb-4">{{ port.status }}</v-list-item-subtitle>

          <v-list-item-subtitle class="mb-4">Subscriptions:</v-list-item-subtitle>
          <v-chip-group column>
            <v-chip class="mb-4" v-for="(subscription, subIndex) in port.subscriptions" :key="subIndex">
              {{ subscription }}
            </v-chip>
          </v-chip-group>

          <v-list-item-subtitle class="mb-4">Subscribed To:</v-list-item-subtitle>
          <v-chip-group column>
            <v-chip class="mb-4" v-for="(subscribed, subToIndex) in port.subscribed_to" :key="subToIndex">
              {{ subscribed }}
            </v-chip>
          </v-chip-group>

          <v-text-field v-model="portValues[index]" label="Send value"></v-text-field>

          <v-list-item-action>
            <v-btn @click="openSerialPort({ name: port.name })" variant="plain">
              Open
            </v-btn>
            <v-btn @click="closeSerialPort(port.name)" class="ml-4" variant="plain">
              Close
            </v-btn>
            <v-btn @click="sendToSerialPort(port.name, portValues[index])" class="ml-4" variant="plain">
              Send
            </v-btn>
          </v-list-item-action>

          <v-text-field v-model="subscriptionInputs[index].subscribeTo" label="Subscribe to"></v-text-field>
          <v-btn @click="subscribe(subscriptionInputs[index].subscribeTo, port.name)" class="mb-4">
            Subscribe
          </v-btn>

          <v-text-field v-model="subscriptionInputs[index].unsubscribeFrom" label="Unsubscribe from"></v-text-field>
          <v-btn @click="unsubscribe(subscriptionInputs[index].unsubscribeFrom, port.name)" class="mb-4">
            Unsubscribe
          </v-btn>

          <v-divider class="mb-4 mt-4"></v-divider>
        </v-list-item>
      </v-list>

      <v-text-field v-model="broadcastValue" label="Enter value to send to all ports"></v-text-field>
      <v-btn @click="sendToAllSerialPorts(broadcastValue)">
        Broadcast
      </v-btn>

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

interface ManagedSerialPort {
  name: string;
  status: Status;
  subscriptions: string[];
  subscribed_to: string[];
}

interface OpenSerialPortOptions {
  name: string;
}

interface SubscriptionInputs {
  subscribeTo: string;
  unsubscribeFrom: string;
}

const managedSerialPorts = ref<ManagedSerialPort[]>([]);
const portValues = ref<string[]>([]);
const broadcastValue = ref<string>('');
const subscriptionInputs = ref<SubscriptionInputs[]>([]);

let unlistenSerialPortsEvent: UnlistenFn;

onMounted(async () => {
  unlistenSerialPortsEvent = await listen('serial_ports_event', (event) => {
    managedSerialPorts.value = event.payload as ManagedSerialPort[];
    portValues.value = managedSerialPorts.value.map(() => ''); subscriptionInputs.value = managedSerialPorts.value.map(() => ({
      subscribeTo: '',
      unsubscribeFrom: ''
    }));
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

const sendToSerialPort = (name: string, value: string) => {
  invoke('send_to_serial_port', { name, value })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
};

const sendToAllSerialPorts = (value: string) => {
  invoke('send_to_all_serial_ports', { value })
    .then((response) => {

    })
    .catch((error) => {
      console.error(error);
    });
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
</script>
