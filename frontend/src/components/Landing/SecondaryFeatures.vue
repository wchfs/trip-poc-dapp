<template>
  <section id="secondary-features" aria-label="Features for simplifying everyday business tasks"
    class="pt-20 pb-14 sm:pb-20 sm:pt-32 lg:pb-32">
    <Container>
      <div class="mx-auto max-w-2xl md:text-center">
        <h2 class="font-display text-3xl tracking-tight text-slate-900 sm:text-4xl">
          Validate ticket by licence plate and maintain your zone
        </h2>
        <p class="mt-4 text-lg tracking-tight text-slate-700">
          Basic set of features for controllers and zone owners
        </p>
      </div>
      <!-- <FeaturesMobile /> -->
      <!-- <FeaturesDesktop /> -->
      <TabGroup as="div" class="hidden lg:mt-20 lg:block" @change="changeTab">
        <TabList class="grid grid-cols-3 gap-x-8">
          <div v-for="(feature, index) in features" class="relative">
            <div :class="clsx('opacity-75 hover:opacity-100')">
              <div :class="clsx(
                'w-9 rounded-lg',
                'bg-blue-600'
              )">
                <!-- <svg aria-hidden="true" className="h-9 w-9" fill="none">
          <feature.icon />
        </svg> -->
              </div>
              <h3 :class="clsx(
                'mt-6 text-sm font-medium',
                'text-blue-600'
              )">
                <Tab class="[&:not(:focus-visible)]:focus:outline-none">
                  <span class="absolute inset-0" />
                  {{ feature.name }}
                </Tab>
              </h3>
              <p class="mt-2 font-display text-xl text-slate-900">
                {{ feature.summary }}
              </p>
              <p class="mt-4 text-sm text-slate-600">{{ feature.description }}</p>
            </div>
          </div>
        </TabList>
        <TabPanels class="relative mt-20 overflow-hidden rounded-4xl bg-slate-200 px-14 py-16 xl:px-16">
          <div class="-mx-5 flex">
            <TabPanel v-for="(feature, index) in features" static class="px-5 transition duration-500 ease-in-out [&:not(:focus-visible)]:focus:outline-none"
              :style="transform">
              <div
                class="w-[52.75rem] overflow-hidden rounded-xl bg-white shadow-lg shadow-slate-900/5 ring-1 ring-slate-500/10">
                <img class="w-full" :src="feature.image" alt="" sizes="52.75rem" />
              </div>
            </TabPanel>
          </div>
          <div class="pointer-events-none absolute inset-0 rounded-4xl ring-1 ring-inset ring-slate-900/10" />
        </TabPanels>
      </TabGroup>
    </Container>
  </section>
</template>

<script setup lang="ts">
import { TabGroup, TabList, TabPanels, TabPanel, Tab } from "@headlessui/vue";
import Container from "@/components/Landing/Container.vue";
import clsx from "clsx";
import { reactive, ref } from 'vue';

const selectedTab = ref(0)
let transform = reactive({
  transform: 'translateX(-100%)',
})

function changeTab(index) {
  let shift = -100 * index;
  transform.transform = 'translateX('+shift+'%)';
  selectedTab.value = index;
}

const features = [
  {
    name: 'GeoZones',
    summary: 'Geojson zone upload',
    description:
      'Upload your city zones to the system be the sole owner. Store it securely in the SQLite database.',
    image: "/images/prtscn_geojson.png",
    icon: "",
  },
  {
    name: 'Validation',
    summary:
      'Validation',
    description:
      'Validate a ticket by licence plate. Give your controllers an easy tool.',
    image: "/images/prtscn_validator.png",
    icon: "",
  },
  {
    name: 'Withdraw',
    summary:
      'Withdraw funds',
    description:
      'Easily withdraw funds to maintain your zone, refresh parking lines, so something good.',
    image: "/images/prtscn_withdraw.png",
    icon: "",
  },
]

</script>