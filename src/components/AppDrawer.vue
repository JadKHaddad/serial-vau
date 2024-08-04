<template>
    <v-navigation-drawer rail permanent app>
        <v-list density="compact" nav>
            <router-link to="/" style="text-decoration: none; color: inherit;">
                <v-list-item prepend-icon="mdi-serial-port" value="serial-port"></v-list-item>
            </router-link>

            <router-link to="/monitor" style="text-decoration: none; color: inherit;">
                <v-list-item prepend-icon="mdi-monitor" value="serial-monitor"></v-list-item>
            </router-link>

        </v-list>
    </v-navigation-drawer>
</template>