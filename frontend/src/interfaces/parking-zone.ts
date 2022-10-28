import type { GeoJSON } from 'geojson';

export type ParkingZone = {
  id: number,
  name: string,
  price: string,
  start_hour: string,
  end_hour: string,
  geo_json: string | GeoJSON,
  owner_address: string,
};
