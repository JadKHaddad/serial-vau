<template>
  <v-app>
    <AppDrawer />

    <v-main scrollable>
      <v-container class="fill-width">
        <v-responsive class="align-center mx-auto">
          <router-view />
        </v-responsive>
      </v-container>
    </v-main>

    <AppFooter />
  </v-app>
</template>

<script lang="ts" setup>
//
</script>

<style>
html {
  overflow-y: auto
}
</style>
