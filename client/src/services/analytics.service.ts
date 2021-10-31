// TODO: https://github.com/hastic/hastic-grafana-app/blob/c67bd8af140105c36f24c875187929869e48e51e/src/panel/graph_panel/services/analytic_service.ts

import { API_URL } from "@/config";
import axios from 'axios';

import { getGenerator } from '@/utils';

import _ from 'lodash';

const ANALYTICS_API_URL = API_URL + "analytics/";

export async function getStatus(): Promise<string> {
  const uri = ANALYTICS_API_URL + `status`;
  const res = await axios.get(uri);
  const data = res['data'] as any;
  return data.status;
}

export function getStatusGenerator(): AsyncIterableIterator<string> {
  return getGenerator<string>(100, getStatus);
}
