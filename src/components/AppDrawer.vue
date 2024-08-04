<template>
    <v-navigation-drawer rail permanent app>
        <v-list density="compact" nav>
            <router-link to="/" style="text-decoration: none; color: inherit;" active-class="v-list-item--active">
                <v-list-item :class="isActiveRoute('/') ? 'v-list-item--active' : ''" prepend-icon="mdi-serial-port"
                    value="serial-port"></v-list-item>
            </router-link>
            <router-link to="/monitor" style="text-decoration: none; color: inherit;"
                active-class="v-list-item--active">
                <v-list-item :class="isActiveRoute('/monitor') ? 'v-list-item--active' : ''" prepend-icon="mdi-monitor"
                    value="serial-monitor"></v-list-item>
            </router-link>
        </v-list>
    </v-navigation-drawer>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';

const route = useRoute();

const isActiveRoute = (path: string) => {
    return route.path === path;
};
</script>
