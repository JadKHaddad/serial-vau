<template>
  <v-container class="fill-height d-colum flex-column pa-sm--0">
    <h1 class="text-h3 text-center mb-6">Serial Ports</h1>
    <v-row scrollable justify="start">
      <v-col 
        v-for="(port, index) in app.managedSerialPorts"
        :key="index"
        cols="12"
        md="6"
      >
        <v-card variant="outlined">
          <SerialPort :port="port"></SerialPort>
        </v-card>
      </v-col>
    </v-row>

    <v-container fluid>
      <v-sheet
        color="transparent"
        class="d-flex justify-center align-center"
        max-width="600"
        style="margin: auto"
      >
        <v-text-field
          autofocus
          messages="Okay then"
          hide-details="auto"
          max-width="500"
          placeholder="example: lights off"
          variant="outlined"
          class="text-none"
          v-model="broadcastValue"
          label="Send Broadcast Message"
          append-icon="mdi-send"
          @keydown.enter.prevent="sendBroadCastMessageToAllPorts"
          @click:append="sendBroadCastMessageToAllPorts"
          @click:clear="clearBroadcastValue"
          clearable
        >
          <template v-slot:message>
            <span class="hidden-sm-and-down">
              Serial Vau can have some errors. Please report immediately (*o*)
            </span>

            <span class="hidden-md-and-up">
              Please report errors immediately (*o*)
            </span>
          </template>
        </v-text-field>
      </v-sheet>
    </v-container>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useAppStore } from "@/stores/app";

const app = useAppStore();

const broadcastValue = ref<string>("");

const clearBroadcastValue = () => {
  broadcastValue.value = "";
};

const sendBroadCastMessageToAllPorts = () => {
  app.sendToAllSerialPorts(broadcastValue.value);
  clearBroadcastValue();
};
</script>
