// TODO: https://github.com/hastic/hastic-grafana-app/blob/c67bd8af140105c36f24c875187929869e48e51e/src/panel/graph_panel/services/analytic_service.ts

import { API_URL } from "@/config";
import axios from 'axios';

import _ from 'lodash';

const METRICS_API_URL = API_URL + "metric/";

export async function getMetrics(from: number, to: number, step: number) {
  if(from >= to) {
    throw new Error("`from` can`t be less than `to`");
  }
  if(step < 1) {
    throw new Error("`step` can`t be less than 1");
  }

  const uri = METRICS_API_URL + `?from=${from}&to=${to}&step=${step}`;
  const res = await axios.get(uri);

  const target = _.keys(res["data"]["data"])[0];
  const values = res["data"]["data"][target];
  return [target, values];
}
