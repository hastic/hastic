import { HasticPod } from './hastic_pod';
import { TimeRange  } from '@/types';

import { Segment } from "@/types/segment";
import { LineTimeSerie } from '@chartwerk/line-pod';
import { SegmentsSet } from '@/types/segment_set';


export type UpdateDataCallback = (range: TimeRange) => Promise<{
  timeserie: LineTimeSerie[],
  bounds: [number, [number, number]][],
  segments: Segment[]
}>;

import * as _ from 'lodash';


export class AnomalyPod extends HasticPod<UpdateDataCallback> {

  _bounds: [number, [number, number]][];

  constructor(
    el: HTMLElement,
    udc: UpdateDataCallback,
    segmentSet: SegmentsSet<Segment>
  ) {
    super(el, udc, segmentSet)
    this.fetchData();
  }

  // TODO: implement renderMetrics

  public fetchData(): void {
    let to = Math.floor(Date.now() / 1000);
    let from = to - 50000; // -50000 seconds

    if(!(this.state.xValueRange[0] == 0 && this.state.xValueRange[1] == 1)) {
      [from, to] = this.state?.xValueRange;
    }

    this.udc({ from, to })
      .then(resp => {
        this.updateSegments(resp.segments);
        this.updateBounds(resp.bounds);
        this.updateData(resp.timeserie, undefined, true);
        
      })
      .catch(() => { /* set "error" message */ })
  }

  renderMetrics() {
    this.renderBounds()
    super.renderMetrics();
    
  }

  updateBounds(bounds: [number, [number, number]][]) {
    this._bounds = bounds;
  }

  renderBounds() {
    
    // TODO: check the case when this._bounds == undefined
    if(this._bounds == undefined) {
      return;
    }

    const pointsUp = this._bounds.map(([t, [u, _]]) => [t, u])
    const pointsDown = this._bounds.map(([t, [_, l]]) => [t, l]);

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
  }

}