/**
 * CARTESI MACHINE TYPES
 */
import type { GeoJSON } from 'geojson';

export type InspectResponse = {
  status: CompletionStatus,
  exception_payload: Payload,
  reports: Report[],
  metadata: InspectMetadata,
};

export enum CompletionStatus {
  Accepted = 'Accepted',
  Rejected = 'Rejected',
  Exception = 'Exception',
  MachineHalted = 'MachineHalted',
  CycleLimitExceeded = 'CycleLimitExceeded',
  TimeLimitExceeded = 'TimeLimitExceeded',
}

export type InspectMetadata = {
  active_epoch_index: number,
  current_input_index: number,
};

export type Payload = string;

export type Report = {
  payload: Payload,
};

export type Error = string;

/**
 * END CARTESI MACHINE TYPES
 */

export type InspectRequest = {
  endpoint: 'get_zones' | 'check_point_in_zones',
  payload: string,
};

export type InspectGetZoneReport = {
  geo_json: string,
  id: number,
  owner_address: string,
  price_per_minute: number,
};

export type InspectGetZonesReport = InspectGetZoneReport[];
