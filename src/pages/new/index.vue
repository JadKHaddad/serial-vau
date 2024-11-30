<template>
    <div class="flex flex-1 flex-col gap-4 p-4">
        <div class="grid auto-rows-min gap-4 md:grid-cols-3">
            <div v-for="index in 3" :key="index"
                class="aspect-video rounded-xl bg-transparent border flex flex-col justify-between border-double border-white p-4">
                <div v-if="schema[index]" class="flex flex-col h-full">

                    <div class="flex justify-between items-start mb-2">
                        <div class="text-2xl flex gap-2 mt-1 uppercase">
                            <X @click="unSelectPortSchema(index)" color="white" class="mt-1 cursor-pointer"></X>
                            {{ schema[index] }}
                        </div>

                        <div class="flex items-center gap-2 uppercase">
                            {{ findPortByName(schema[index])?.status.type }}
                            <Circle :color="isPortOpen(schema[index]) ? 'green' : 'red'" />
                        </div>
                    </div>
                    <div class="mt-auto flex justify-center gap-2">
                        <Button @click="selectSerialPort(schema[index])" class="uppercase bg-black"
                            variant="outline">Select</Button>
                        <Button class="uppercase bg-black" variant="outline">Subscribe</Button>
                        <Button @click="toggleStatusType(schema[index])" class="uppercase bg-black"
                            variant="outline">Open /
                            Close</Button>
                        <Button class="uppercase bg-black" variant="outline">Toggle Read</Button>
                    </div>
                </div>
                <div v-else class=" flex items-center w-100 h-100 justify-center">
                    <DropdownMenu>
                        <DropdownMenuTrigger><Button :class="temporyButtonColor" size="default" class="flex">Select an
                                Serial Port</Button></DropdownMenuTrigger>
                        <DropdownMenuContent class="text-gray-900 bg-black w-56">
                            <DropdownMenuLabel>Select an Serial Port</DropdownMenuLabel>
                            <DropdownMenuSeparator class="bg-gray-800" />

                            <DropdownMenuItem v-for="port of managedSerialPorts" class="cursor-pointer"
                                @click="setSerialPortSchema(index, port)">
                                {{ port.name }}
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            </div>
        </div>

        <div class="h-[54vh] rounded-md bg-transparent border-double border border-white overflow-hidden">
            <div v-bind="containerProps" class="h-full overflow-y-auto bg-gray-500/5 p-2">
                <div v-bind="wrapperProps">
                    <div v-for="{ data, index } in list" :key="index" class="m-3" :style="{
                        height: `1rem`,
                        display: 'flex',
                        justifyContent: 'center',
                        alignItems: 'center',

                    }">

                        <span> {{ packetDataTemplate(data) }}</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="w-100"><Input /></div>
    </div>
</template>

<script lang="ts" setup>


import { useAppStore } from '@/stores/app';
import { storeToRefs } from 'pinia';

import { Circle } from 'lucide-vue-next'

import { useLocalStorage, useVirtualList } from "@vueuse/core"
import { ManagedSerialPort, StatusType } from '@/models/managed-serial-port';
import { X } from 'lucide-vue-next';

import { useListener } from "@/utlis/listener";
import { PacketData } from '@/models/intern/packet-data';
import { IncomingPacket, OutgoingPacket, PacketDirectionType, PacketOriginType } from '@/models/packet';

const { setupListeners, cleanupListeners } = useListener();


const selectedPortName = useLocalStorage<string>('selected-port', null)

const temporyButtonColor = 'bg-black' as const

type PreviewPorts = Record<number, undefined | ManagedSerialPort["name"]>

const schema = useLocalStorage<PreviewPorts>('user-storage', {
    0: undefined,
    1: undefined,
    2: undefined,
})




const portDisplayPacketsLimits = ref<Record<string, number>>({});
const index: Ref = ref<number>()

const appStore = useAppStore()



const { openSerialPort, toggleReadState, closeSerialPort } = appStore
const { managedSerialPorts, packets } = storeToRefs(appStore)



const limitedPackets = (): PacketData[] => {
    if (!packets.value || !selectedPortName.value) return [];


    const packetLimit = portDisplayPacketsLimits.value[selectedPortName.value] || 1000;

    const data = packets.value[selectedPortName.value];

    return data.slice(Math.max(data.length - packetLimit, 0));
};

const findPortByName = (portname: string): ManagedSerialPort | undefined => {
    return managedSerialPorts.value.find(item => item.name === portname)
}


const selectSerialPort = (portname: string) => {
    console.debug(portname)
    selectedPortName.value = portname;

}


const setSerialPortSchema = (previewPortIndex: number, port: ManagedSerialPort) => {
    schema.value[previewPortIndex] = port.name
}

const unSelectPortSchema = (previewPortIndex: number) => {
    schema.value[previewPortIndex] = undefined
}


const isPortOpen = (portName: string): boolean => {
    return findPortByName(portName)?.status.type === StatusType.Open;
};

const toggleStatusType = (portname: string) => {
    const portData = findPortByName(portname)

    switch (portData?.status.type) {
        case StatusType.Closed:
            openSerialPort(portname, portData.lastUsedOpenOptions)
            break;
        case StatusType.Open:
            closeSerialPort(portname)
    }
}


const packetDataTemplate = (data: PacketData): string => {

    const dateFormat = (date: number): string => {
        return `${new Date(date).toLocaleDateString()}, ${new Date(
            date
        ).toLocaleTimeString()}`;
    };

    const { packetDirection, timestampMillis } = data


    const subscriptionTemplate = (content: OutgoingPacket) => {
        /** @example  */
        return `[ ${dateFormat(timestampMillis)} ] ${PacketOriginType.Subscription.toUpperCase()}: ${content.value}`

    }

    const baseTemplate = (content: OutgoingPacket) => {

        return `[${dateFormat(timestampMillis)}] ${content.packetOrigin.type.toUpperCase()}: ${content.value}`
    }

    const incomingTemplate = (content: IncomingPacket) => {
        return `[${dateFormat(timestampMillis)}] ${PacketDirectionType.Incoming.toUpperCase()}: ${content.line} `
    }

    switch (packetDirection.type) {
        case PacketDirectionType.Incoming:

            /** the content could look like IncomingPacket -> { line: string }  */
            const content = packetDirection.content
            return incomingTemplate(content)

        case PacketDirectionType.Outgoing:
            /** the content could look like IncomingPacket -> {packetOrigin: PacketOrigin value: string}  */
            const packetContent = packetDirection.content
            const packetOriginType = packetContent.packetOrigin.type
            switch (packetOriginType) {
                case PacketOriginType.Broadcast:
                case PacketOriginType.Direct:
                    baseTemplate(packetContent)
                case PacketOriginType.Subscription:
                    const subsContent = packetDirection.content
                    subscriptionTemplate(subsContent)

                default:
                    throw new Error("Unexpected packet PacketOrigin type")
            }


        default:
            throw new Error("Unexpected packet PacketDirection type")
    }

}







const filteredItems = computed<PacketData[]>(() => {
    return limitedPackets()
})

const { list, containerProps, wrapperProps } = useVirtualList(
    filteredItems,
    {
        itemHeight: 10,
        overscan: 20,
    },
)

onMounted(async () => {
    setupListeners();
});

onUnmounted(() => {
    cleanupListeners();
});



</script>