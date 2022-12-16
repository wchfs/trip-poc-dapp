<template>
  <Disclosure
    as="nav"
    class="bg-gray-800 drop-shadow-md"
    v-slot="{ open }"
  >
    <div class="container mx-w-6xl mx-auto">
      <div class="relative flex h-16 items-center justify-between">
        <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
          <!-- Mobile menu button-->
          <DisclosureButton
            class="inline-flex items-center justify-center rounded-md ml-3 p-2 text-gray-400 hover:bg-gray-700 hover:text-white"
          >
            <span class="sr-only">Open main menu</span>
            <Bars3Icon
              v-if="!open"
              class="block h-6 w-6"
              aria-hidden="true"
            />
            <XMarkIcon
              v-else
              class="block h-6 w-6"
              aria-hidden="true"
            />
          </DisclosureButton>
        </div>
        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
          <div class="hidden sm:ml-4 xl:ml-0 sm:block">
            <div class="flex space-x-4">
              <RouterLink
                v-for="(item, index) in menuItems"
                :key="index"
                :to="{ name: item.routeName }"
                :class="[
                  item.active ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'px-3 py-2 rounded-md text-sm font-medium'
                ]"
                :aria-current="item.active ? 'page' : undefined"
              >
                {{ item.displayName }}
              </RouterLink>
            </div>
          </div>
        </div>
      </div>
    </div>

    <DisclosurePanel class="sm:hidden">
      <div class="space-y-1 px-2 pt-2 pb-3">
        <DisclosureButton
          v-for="(item, index) in menuItems"
          :key="index"
          as="div"
          :aria-active="item.active ? 'page' : undefined"
        >
          <RouterLink
            :to="{ name: item.routeName }"
            :class="[
              item.active ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'block px-3 py-2 rounded-md text-base font-medium'
            ]"
          >
            {{ item.displayName }}
          </RouterLink>
        </DisclosureButton>
      </div>
    </DisclosurePanel>
  </Disclosure>
</template>

<script setup lang="ts">
import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/vue';
import { Bars3Icon, XMarkIcon } from '@heroicons/vue/24/outline';
import { onMounted, ref, watch } from "vue";
import { RouterLink, useRouter } from "vue-router";

interface MenuItem {
  displayName: string;
  routeName: string;
  active?: boolean;
}

const menuItems = ref<MenuItem[]>([
  {
    displayName: "Parking DApp",
    routeName: "dapp.home",
  },
  {
    displayName: "My Tickets",
    routeName: "dapp.tickets.my",
  },
  {
    displayName: "My Zones",
    routeName: "dapp.zones.my",
  },
  {
    displayName: "Tickets Validator",
    routeName: "dapp.tickets.validate",
  },
  {
    displayName: "Vouchers",
    routeName: "dapp.vouchers",
  },
]);

function markCurrentRoute(currentRouteName: string) {
  menuItems.value = menuItems.value.map((menuItem: MenuItem) => {
    menuItem.active =
      !!currentRouteName && currentRouteName.toString().startsWith(menuItem.routeName);

    return menuItem;
  });
}

onMounted(() => {
  markCurrentRoute(useRouter().currentRoute.value.name.toString());
});

watch(useRouter().currentRoute, async (currentRoute) => {
  markCurrentRoute(currentRoute.name.toString());
});
</script>
