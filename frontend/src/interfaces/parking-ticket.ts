export type ParkingTicket = {
  id: number,
  license: string,
  longitude: number,
  latitude: number,
  started_at: string,
  owner_address: string,
  purchased_at: string,
  duration: number,
  zone_id: number,
  paid: number,
  to_pay: number,
  status: number,
};
