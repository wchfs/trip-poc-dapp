<template>
  <BaseContainer>
    <Box
      additionalClass="col-span-3 p-0 mb-6"
    >
      <div
        :class="`
          ${showForm ? 'border-b border-gray-200' : ''}
          bg-white
          px-4
          py-5
          sm:px-6
        `"
      >
        <div class="-ml-4 -mt-4 flex flex-wrap items-center justify-between sm:flex-nowrap">
          <div class="ml-4 mt-4">
            <h3 class="text-lg font-medium leading-6 text-gray-900">Proposal create</h3>
            <p class="mt-1 text-sm text-gray-500">
              You can create your own proposals to manage things like zone price or even create a new zone
            </p>
          </div>
          <div class="ml-4 mt-4 flex-shrink-0">
            <TButton
              v-if="!showForm"
              type="button"
              color="indigo"
              @click="showForm = !showForm"
            >
              Create new proposal
            </TButton>
          </div>
        </div>
      </div>
      <div
        v-if="showForm"
        class="py-6 px-6 md:py-10 md:px-10"
      >
        <ProposalCreateCard @cancel="showForm = false"/>
      </div>
    </Box>
  </BaseContainer>
</template>

<script setup lang="ts">
import Box from '@/components/Box/Box.vue';
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import { ref } from 'vue';
import ProposalCreateCard from '@/views/Proposals/List/Create/ProposalCreateCard.vue';
import TButton from '@/components/Controls/Button/TButton.vue';

const showForm = ref(false);
</script>
