import { API_URL } from "@/config";
import axios from 'axios';

const METRICS_API_URL = API_URL + "metrics";

export default async function getMetrics(from: number, to: number, step: number) {
  let uri = METRICS_API_URL + `?from=${from}`;
  let res = axios.get(METRICS_API_URL);
}
