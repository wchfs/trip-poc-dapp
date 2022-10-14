export interface Proposal {
  id: number;
  title: string;
  description: string;
  status: 'pending' | 'approved' | 'rejected';
  votes_up: number;
  votes_down: number;
  created_at: string;
}
