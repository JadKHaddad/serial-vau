<script setup lang="ts">
import { Check, ChevronsUpDown, GalleryVerticalEnd, Search } from 'lucide-vue-next'
import { ref } from 'vue'

const data = {
    navMain: [
        {
            title: 'Navigation',
            url: '#',
            items: [
                { title: 'Installation', url: '/new/test' },
                { title: 'Project Structure', url: '#' },
            ],
        },
    ],
}

const dropdownOpen = ref(false)
const search = ref('')

function toggleDropdown() {
    dropdownOpen.value = !dropdownOpen.value
}
</script>

<template>
    <SidebarProvider>
        <Sidebar>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <DropdownMenu>
                            <DropdownMenuTrigger as-child>
                                <SidebarMenuButton size="lg"
                                    :class="{ 'data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground': dropdownOpen }">
                                    <div
                                        class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                                        <GalleryVerticalEnd class="size-4" />
                                    </div>
                                    <div class="flex flex-col gap-0.5 leading-none">
                                        <span class="">Serial Vau</span>
                                    </div>
                                </SidebarMenuButton>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent v-if="dropdownOpen" class="w-[--radix-dropdown-menu-trigger-width]"
                                align="start">
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </SidebarMenuItem>
                </SidebarMenu>

                <form @submit.prevent>
                    <SidebarGroup class="py-0">
                        <SidebarGroupContent class="relative">
                            <Label for="search" class="sr-only">Search</Label>
                            <SidebarInput id="search" v-model="search" placeholder="Search the navigation..."
                                class="pl-8" />
                            <Search
                                class="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 select-none opacity-50" />
                        </SidebarGroupContent>
                    </SidebarGroup>
                </form>
            </SidebarHeader>

            <SidebarContent>
                <SidebarGroup v-for="item in data.navMain" :key="item.title">
                    <SidebarGroupLabel>{{ item.title }}</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>

            <SidebarRail />
        </Sidebar>

        <SidebarInset>
            <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
                <SidebarTrigger class="-ml-1" />
                <Separator orientation="vertical" class="mr-2 h-4" />
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem class="hidden md:block">
                            <BreadcrumbLink href="#">
                                Building Your Application
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator class="hidden md:block" />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Data Fetching</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </header>

            <slot>

            </slot>


        </SidebarInset>
    </SidebarProvider>
</template>