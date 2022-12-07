<template>
  <ProposalCreateBox/>
  <BaseContainer>
    <ProposalListItem
      v-for="proposal of proposals" :key="proposal.id"
      :proposal="proposal"
    />
  </BaseContainer>
</template>

<script setup lang="ts">
import BaseContainer from '@/components/Containers/BaseContainer.vue';
import { useProposalStore } from '@/stores/proposal';
import { storeToRefs } from 'pinia';
import { onMounted } from 'vue';
import ProposalCreateBox from '@/views/App/Proposals/List/Create/ProposalCreateBox.vue';
import ProposalListItem from '@/views/App/Proposals/List/ProposalListItem.vue';

const proposalStore = useProposalStore();
const {
  proposals
} = storeToRefs(proposalStore);

onMounted(() => {
  proposalStore.fetchProposals();
});


</script>
