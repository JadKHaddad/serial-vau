<template>
  <v-container fluid style="height: calc(100%)">
    <h1 class="text-h3 text-center mb-6">Serial Ports</h1>

    <v-responsive class="pa-5 scrollable-cards" style="height: calc(83%)">
      <v-row>
        <v-col
          v-for="(port, index) in app.managedSerialPorts"
          :key="index"
          cols="12"
          md="4"
        >
          <v-lazy
            :options="{
              threshold: 0.5,
            }"
          >
            <v-card
              variant="outlined"
              class="pa-4 fill-height d-flex flex-column"
            >
              <SerialPort :port="port"></SerialPort>
            </v-card>
          </v-lazy>
        </v-col>
      </v-row>
    </v-responsive>

    <v-row class="pa-2" style="height: 4%">
      <v-col cols="12">
        <v-text-field
          v-model="broadcastMessage"
          class="mx-16"
          label="Input text here"
          variant="outlined"
          density="comfortable"
          append-icon="mdi-send"
          @keydown.enter.prevent="sendBroadcastMessage()"
          @click:append="sendBroadcastMessage"
          @click:clear="clearBroadcastMessage"
          clearable
        ></v-text-field>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useAppStore } from "@/stores/app";

const app = useAppStore();

const broadcastMessage = ref("");

const clearBroadcastMessage = () => {
  broadcastMessage.value = "";
};

const sendBroadcastMessage = () => {
  if (broadcastMessage.value) {
    app.sendToAllSerialPorts(broadcastMessage.value);
    broadcastMessage.value = "";
  }
};
</script>

<style scoped>
.scrollable-cards {
  overflow-y: auto; /* Enable vertical scrolling */
  padding: 10px;
}
</style>
