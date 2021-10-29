import { API_URL } from "@/config";
import { Segment, SegmentId } from "@/types/segment";
import axios from 'axios';

import _ from 'lodash';

const SEGMENTS_API_URL = API_URL + "segments/";

export async function getSegments(from: number, to: number) {
  if(from >= to) {
    throw new Error("`from` can`t be less than `to`");
  }

  const uri = SEGMENTS_API_URL + `?from=${from}&to=${to}`;
  const res = await axios.get(uri);

  const target = _.keys(res["data"]["data"])[0];
  const values = res["data"]["data"][target];
  return [target, values];
}

export async function postSegment(segment: Segment): Promise<SegmentId> {
  delete segment.id; // because we post a new segment
  const resp = await axios.post(SEGMENTS_API_URL, segment);
  console.log(resp);
  return 10 + "";
}
