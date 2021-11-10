import { API_URL } from "@/config";
import { Segment, SegmentId } from "@/types/segment";
import axios from 'axios';

import _ from 'lodash';

const SEGMENTS_API_URL = API_URL + "segments/";
const ANALYTICS_API_URL = API_URL + "analytics/";

export async function getSegments(from: number, to: number, withLabeling = true): Promise<Segment[]> {
  if(from >= to) {
    throw new Error("`from` can`t be less than `to`");
  }

  let result = [];
  if (withLabeling) {
    const uri = SEGMENTS_API_URL + `?from=${from}&to=${to}`;
    const res = await axios.get(uri);
    result = res["data"] as any[];
  }

  const uriAnalytics = ANALYTICS_API_URL + `?from=${from}&to=${to}`;
  const resAnalytics = await axios.get(uriAnalytics);

  const resultAnalytics = resAnalytics["data"] as any[];

  return result.concat(resultAnalytics).map(Segment.fromObject);
}

export async function postSegment(segment: Segment): Promise<SegmentId> {
  const segObj = segment.toObject();
  segObj.id = undefined; // because we post a new segment. It's a hack

  const resp = await axios.post(SEGMENTS_API_URL, segObj);
  return resp['data']['id'];
}

export async function deleteSegment(from: number, to: number): Promise<number> {
  const uri = SEGMENTS_API_URL + `?from=${from}&to=${to}`;
  const resp = await axios.delete(uri);
  return resp['data']['count'];
}
