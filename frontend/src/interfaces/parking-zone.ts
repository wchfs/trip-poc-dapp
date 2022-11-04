import type { GeoJSON } from 'geojson';

export type ParkingZone = {
  id: number,
  name: string,
  price: string,
  geo_json: string | GeoJSON,
  owner_address: string,
};
