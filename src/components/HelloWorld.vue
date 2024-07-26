<template>
  <v-container class="fill-height">
    <v-responsive class="align-center fill-height mx-auto" max-width="900">
      <v-img class="mb-4" height="150" src="@/assets/logo.png" />
      <v-text-field label="Enter your name" v-model="name"></v-text-field>
      <v-btn color="primary" @click="greet">Greet</v-btn>
      <p v-if="greeting">{{ greeting }}</p>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';

// Create reactive states
const name = ref('');
const greeting = ref('');

// Function to call the greet command
const greet = () => {
  invoke('greet', { name: name.value })
    .then((response) => {
      greeting.value = response as string;
      console.log(response);
    })
    .catch((error) => {
      console.error(error);
    });
};
</script>
