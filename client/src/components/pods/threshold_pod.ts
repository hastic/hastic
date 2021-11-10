import { HasticPod } from './hastic_pod';
import { TimeRange  } from '@/types';

import { Segment } from "@/types/segment";
import { LineTimeSerie } from '@chartwerk/line-pod';
import { SegmentsSet } from '@/types/segment_set';


export type UpdateDataCallback = (range: TimeRange) => Promise<{
  timeserie: LineTimeSerie[],
  segments: Segment[]
}>;


export class ThresholdPod extends HasticPod<UpdateDataCallback> {

  constructor(
    el: HTMLElement,
    udc: UpdateDataCallback,
    segmentSet: SegmentsSet<Segment>
  ) {
    super(el, udc, segmentSet)

    this.fetchData();
  }


  public fetchData(): void {
    let to = Math.floor(Date.now() / 1000);
    let from = to - 50000; // -50000 seconds

    if(!(this.state.xValueRange[0] == 0 && this.state.xValueRange[1] == 1)) {
      [from, to] = this.state?.xValueRange;
    }

    this.udc({ from, to })
      .then(resp => {
        this.updateSegments(resp.segments);
        this.updateData(resp.timeserie, undefined, true);
      })
      .catch(() => { /* set "error" message */ })
  }


  protected updateSegments(segments: Segment[]): void {
    this.segmentSet.clear();
    this.segmentSet.setSegments(segments);
  }

}