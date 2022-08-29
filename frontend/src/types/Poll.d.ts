type Poll = {
  poll_id: number;
  created_at: number;
  poll_owner: string;
  poll_status: string;
  candidates: Array<Candidate>;
  results: { [key: string]: number };
};
