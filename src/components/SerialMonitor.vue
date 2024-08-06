<template>
    <!-- 
     <v-container>
       
        <v-tabs v-model="selectedPort" background-color="primary" dark>
            <v-tab v-for="(packetData, portName) in app.packetDictionary" :key="portName">
                {{ portName }}
            </v-tab>
        </v-tabs>


        <v-tabs-items v-model="selectedPort">
            <v-tab-item v-for="(packetData, portName) in app.packetDictionary" :key="portName">
                <v-list>
                    <v-list-item-group v-if="packetData.length">
                        <v-list-item v-for="(packet, index) in packetData" :key="index">
                            <v-list-item-content>
                                <v-list-item-title>{{ packet.line }}</v-list-item-title>
                                <v-list-item-subtitle>{{ new Date(packet.timestampMillis).toLocaleString()
                                    }}</v-list-item-subtitle>
                            </v-list-item-content>
                        </v-list-item>
                    </v-list-item-group>
                    <v-list-item v-else>
                        <v-list-item-content>No data available</v-list-item-content>
                    </v-list-item>
                </v-list>
            </v-tab-item>
        </v-tabs-items>
    </v-container> 
    -->
</template>

<script lang="ts" setup>
// import { useAppStore } from '@/stores/app';
// // TODO: we need a direction of the incoming packet. 
// // we need to have access to the managed serial ports to display if its open or not and display other stats
// // no write access required because we use tauri events. ewe call tauri functions from here if we want then we get an evnet in the app
// const app = useAppStore()
// const selectedPort = ref<string>('');
// const portNames = computed(() => Object.keys(app.packetDictionary));
// if (portNames.value.length > 0) {
//     selectedPort.value = portNames.value[0];
// }

</script>