import { API_URL } from "@/config";
import axios from 'axios';

const METRICS_API_URL = API_URL + "metric/";

export async function getMetrics(from: number, to: number, step: number) {
  const uri = METRICS_API_URL + `?from=${from}&to=${to}&step=${step}`;
  const res = axios.get(uri);
  return res;
}
