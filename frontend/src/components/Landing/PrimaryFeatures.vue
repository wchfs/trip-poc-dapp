<template>
  <section
    id="features"
    aria-label="Features for drivers"
    class="relative overflow-hidden bg-blue-600 pt-20 pb-28 sm:py-32"
  >
    <img
      class="absolute top-1/2 left-1/2 max-w-none translate-x-[-44%] translate-y-[-42%]"
      src="/images/background-features.jpg"
      alt=""
      width="2245"
      height="1636"
      unoptimized
    />
    <Container className="relative">
      <div class="max-w-2xl md:mx-auto md:text-center xl:max-w-none">
        <h2
          class="font-display text-3xl tracking-tight text-white sm:text-4xl md:text-5xl"
        >
          A set of features for drivers
        </h2>
        <p class="mt-6 text-lg tracking-tight text-blue-100">
          Use your location to determine which zone you are in and how much money you owe.
          Check the parking ticket cost prior to purchasing with a free-of-charge state
          check request. Pay with your wallet and see your ticket validity
        </p>
      </div>
      <TabGroup
        as="div"
        class="mt-16 grid grid-cols-1 items-center gap-y-2 pt-10 sm:gap-y-6 md:mt-20 lg:grid-cols-12 lg:pt-0"
        vertical
      >
        <div
          class="-mx-4 flex overflow-x-auto pb-4 sm:mx-0 sm:overflow-visible sm:pb-0 lg:col-span-5"
        >
          <TabList
            class="relative z-10 flex gap-x-4 whitespace-nowrap px-4 sm:mx-auto sm:px-0 lg:mx-0 lg:block lg:gap-x-0 lg:gap-y-1 lg:whitespace-normal"
          >
            <div
              v-for="(feature, index) in features"
              :class="
                clsx(
                  'group relative rounded-full py-1 px-4 lg:rounded-r-none lg:rounded-l-xl lg:p-6',
                  index !== index
                    ? 'bg-white lg:bg-white/10 lg:ring-1 lg:ring-inset lg:ring-white/10'
                    : 'hover:bg-white/10 lg:hover:bg-white/5'
                )
              "
            >
              <h3>
                <Tab
                  :class="
                    clsx(
                      'font-display text-lg [&:not(:focus-visible)]:focus:outline-none',
                      index !== index
                        ? 'text-blue-600 lg:text-white'
                        : 'text-blue-100 hover:text-white lg:text-white'
                    )
                  "
                >
                  <span
                    class="absolute inset-0 rounded-full lg:rounded-r-none lg:rounded-l-xl"
                  />
                  {{ feature.title }}
                </Tab>
              </h3>
              <p
                :class="
                  clsx(
                    'mt-2 hidden text-sm lg:block',
                    index !== index
                      ? 'text-white'
                      : 'text-blue-100 group-hover:text-white'
                  )
                "
              >
                {{ feature.description }}
              </p>
            </div>
          </TabList>
        </div>
        <TabPanels class="lg:col-span-7">
          <TabPanel v-for="feature in features">
            <div class="relative sm:px-6 lg:hidden">
              <div
                class="absolute -inset-x-4 top-[-6.5rem] bottom-[-4.25rem] bg-white/10 ring-1 ring-inset ring-white/10 sm:inset-x-0 sm:rounded-t-xl"
              />
              <p class="relative mx-auto max-w-2xl text-base text-white sm:text-center">
                {{ feature.description }}
              </p>
            </div>
            <div
              class="mt-10 w-[45rem] overflow-hidden rounded-xl bg-slate-50 shadow-xl shadow-blue-900/20 sm:w-auto lg:mt-0 lg:w-[67.8125rem]"
            >
              <img
                class="w-full"
                :src="feature.image"
                alt=""
                priority
                sizes="(min-width: 1024px) 67.8125rem, (min-width: 640px) 100vw, 45rem"
              />
            </div>
          </TabPanel>
        </TabPanels>
      </TabGroup>
    </Container>
  </section>
</template>

<script setup lang="ts">
import { Tab, TabGroup, TabList, TabPanels, TabPanel } from "@headlessui/vue";
import Container from "@/components/Landing/Container.vue";
import clsx from "clsx";
import { ref } from "vue";

const features = [
  {
    title: "Blockchain Verification",
    description:
      "Use your GPS or select your parking zone. Ask the dapp for verification. Everything is transparent.",
    image: "/images/prtscn_blockchain_verification.png",
  },
  {
    title: "See active ticket stored in the SQLite database",
    description:
      "All of your receipts organized into one place, as long as you don't mind typing in the data by hand.",
    image: "/images/prtscn_active_tickets.png",
  },
  {
    title: "Use Any Wallet",
    description: "With our implementation you can use any web3 wallet.",
    image: "/images/prtscn_any_wallet.png",
  },
  {
    title: "Use on any device",
    description: "This implementation is responsive and it can be used on any device.",
    image: "/images/prtscn_any_device.png",
  },
];
</script>
