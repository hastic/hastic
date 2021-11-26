export type TimeRange = { from: number, to: number };

export type AnomalyHSR = {
  seasonality: number,
  timestamp: number,
  ts: [number, number, [number, number]][]
};
