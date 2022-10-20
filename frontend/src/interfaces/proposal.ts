export interface Proposal {
  id: number;
  title: string;
  description: string;
  status: 'pending' | 'approved' | 'rejected';
  votes_up: number;
  votes_down: number;
  created_at: string;
}

export interface ProposalVote {
  wallet_address: string,
  type: 'up' | 'down',
  transaction_address: string,
  created_at: string,
}
