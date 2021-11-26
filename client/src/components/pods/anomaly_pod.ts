import { HasticPod } from './hastic_pod';
import { AnomalyHSR, TimeRange  } from '@/types';

import { Segment } from "@/types/segment";
import { LineTimeSerie } from '@chartwerk/line-pod';
import { SegmentsSet } from '@/types/segment_set';


export type UpdateDataCallback = (range: TimeRange) => Promise<{
  timeserie: LineTimeSerie[],
  hsr: AnomalyHSR,
  segments: Segment[]
}>;

import * as _ from 'lodash';


export class AnomalyPod extends HasticPod<UpdateDataCallback> {

  private _hsr: AnomalyHSR;

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
        this.updateHSR(resp.hsr);
        this.updateData(resp.timeserie, undefined, true);
      })
      .catch(() => { /* set "error" message */ })
  }

  renderMetrics() {
    this.renderHSR()
    super.renderMetrics();
  }

  updateHSR(hsr: AnomalyHSR) {
    this._hsr = hsr;
  }

  renderHSR() {
    
    // TODO: check the case when this._bounds == undefined
    if(this._hsr == undefined) {
      return;
    }

    
    const pointsUp = this._hsr.ts.map(([t, v, [p, q]]) => [t, q]);
    const pointsDown = this._hsr.ts.map(([t, v, [p, q]]) => [t, p]);

    const points = pointsUp.reverse().concat(pointsDown)
      .map(([t, v]) => `${this.xScale(t)},${this.yScale(v)}`)
      .join(' ')

    this.metricContainer
      .append('g')
      .append('polygon')
      .attr('fill', 'green')
      .attr('stroke', 'none')
      .attr('fill-opacity', 0.2)
      .attr('pointer-events', 'none')
      .attr('points', points);

    // TODO: render timestamp
    // TODO: render seasonality grid
  }

}