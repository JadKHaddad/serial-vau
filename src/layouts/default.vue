<template>
  <template v-if="isNewUi">
    <AppSidebar>
      <router-view />
    </AppSidebar>

  </template>


  <v-app v-else>
    <AppDrawer />
    <v-main scrollable>
      <router-view />
    </v-main>
    <AppFooter />
  </v-app>
</template>
<script setup lang="ts">

import router from "@/router"

import AppSidebar from "@/components/new/AppSidebar.vue";


const isNewUi = computed<boolean>(() => {
  console.debug("newUI", router.currentRoute.value.fullPath)
  return router.currentRoute.value.fullPath.includes('new')
})




router.currentRoute




</script>