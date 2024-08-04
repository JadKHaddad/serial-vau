<template>
  <v-app>
    <AppDrawer />

    <v-main>
      <v-container class="fill-height fill-width">
        <v-responsive class="align-center fill-height fill-width mx-auto">
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
