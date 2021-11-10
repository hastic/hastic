// TODO: https://github.com/hastic/hastic-grafana-app/blob/c67bd8af140105c36f24c875187929869e48e51e/src/panel/graph_panel/services/analytic_service.ts

import { API_URL } from "@/config";
import axios from 'axios';

import { getGenerator } from '@/utils';

import _ from 'lodash';
import { 
  AnalyticUnitType, AnlyticUnitConfig,
  PatternConfig, ThresholdConfig, AnomalyConfig 
} from "@/types/analytic_units";

const ANALYTICS_API_URL = API_URL + "analytics/";

export async function getStatus(): Promise<string> {
  const uri = ANALYTICS_API_URL + `status`;
  const res = await axios.get(uri);
  const data = res['data'] as any;
  return data.status;
}

export async function getConfig(): Promise<[AnalyticUnitType, AnlyticUnitConfig]> {
  const uri = ANALYTICS_API_URL + `config`;
  const res = await axios.get(uri);

  const data = res['data'];

  let analyticUnitType = AnalyticUnitType.ANOMALY;
  let analyticUnitConfig = undefined;
  if(data['Pattern'] !== undefined) {
    analyticUnitType = AnalyticUnitType.PATTERN;
    analyticUnitConfig = data['Pattern'] as PatternConfig
  }
  if(data['Threshold'] !== undefined) {
    analyticUnitType = AnalyticUnitType.THRESHOLD;
    analyticUnitConfig = data['Threshold'] as ThresholdConfig
  }
  if(data['Anomaly'] !== undefined) {
    analyticUnitType = AnalyticUnitType.ANOMALY;
    analyticUnitConfig = data['Anomaly'] as AnomalyConfig
  }

  if(analyticUnitConfig === undefined) {
    throw new Error("unknows config type" + _.keys(data));
  }

  return [analyticUnitType, analyticUnitConfig];
}

export async function patchConfig(patchObj: any) {
  const uri = ANALYTICS_API_URL + `config`;
  await axios.patch(uri, patchObj);
}

export function getStatusGenerator(): AsyncIterableIterator<string> {
  return getGenerator<string>(100, getStatus);
}

export async function getHSR(from: number, to: number): Promise<[number, number][]> {
  if(from >= to) {
    throw new Error("`from` can`t be less than `to`");
  }

  const uri = ANALYTICS_API_URL + `hsr/?from=${from}&to=${to}`;
  const res = await axios.get(uri);

  const values = res["data"]["TimeSerie"];
  console.log(values);
  
  return values as [number, number][];
}
