import { HasticPod } from './hastic_pod';
import { AnomalyHSR, TimeRange  } from '@/types';

import { Segment } from "@/types/segment";
import { LineTimeSerie } from '@chartwerk/line-pod';
import { SegmentsSet } from '@/types/segment_set';

import * as _ from 'lodash';


export type UpdateDataCallback = (range: TimeRange) => Promise<{
  timeserie: LineTimeSerie[],
  hsr: AnomalyHSR,
  segments: Segment[]
}>;

export type SetSeasonalityCallback = (from: number, to: number) => void;


export class AnomalyPod extends HasticPod<UpdateDataCallback> {

  private _ssc: SetSeasonalityCallback;

  private _hsr: AnomalyHSR;

  private _zKeyIsDown: boolean;

  private _labelSeasonality: boolean;

  constructor(
    el: HTMLElement,
    udc: UpdateDataCallback,
    ssc: SetSeasonalityCallback,
    segmentSet: SegmentsSet<Segment>
  ) {
    super(el, udc, segmentSet);
    this._zKeyIsDown = false;
    this._ssc = ssc;

    window.addEventListener("keydown", e => {
      if(e.code == "KeyZ") {
        this._zKeyIsDown = true;
      }
    });

    window.addEventListener("keyup", (e) => {
      if(e.code == "KeyZ") {
        this._zKeyIsDown = false;
      }
    });

    this.fetchData();
  }

  protected onBrushStart(): void {
    if(this._zKeyIsDown) {
      this._labelSeasonality = true;
      this.svg.select('.selection')
        .attr('fill', 'orange');
    }

    // TODO: move to state
    this.isBrushing === true;
    const selection = this.d3.event.selection;
    if(selection !== null && selection.length > 0) {
      this.brushStartSelection = this.d3.event.selection[0];
    }
    this.onMouseOut();
  }

  protected onBrushEnd(): void {
    console.log("END");
    if(!this._labelSeasonality) {
      super.onBrushEnd();
    } else {
      const extent = this.d3.event.selection;
      this.isBrushing === false;
      if(extent === undefined || extent === null || extent.length < 2) {
        return;
      }
      this.chartContainer
        .call(this.brush.move, null);

      const startTimestamp = this.xScale.invert(extent[0]);
      const endTimestamp = this.xScale.invert(extent[1]);

      if(this._labelSeasonality) {
        this._ssc(startTimestamp, endTimestamp);
        this._labelSeasonality = false;
      }
    }
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

    // seasonality grid
    let ts = this._hsr.timestamp;
    this._renderHSRGridLine(ts, true);
    ts -= this._hsr.seasonality;
    while(ts > this.state.xValueRange[0]) {
      this._renderHSRGridLine(ts, false);
      ts -= this._hsr.seasonality;
    }
  }

  _renderHSRGridLine(timestamp, head) {
    const x = this.xScale(timestamp);
    
    this.metricContainer
      .append('line')
      .attr('x1', x)
      .attr('x2', x)
      .attr('y1', 0)
      .attr('y2', this.height)
      .attr("style", `stroke:blue;stroke-width: ${head ? 2 : 1}`)
      .attr('opacity', head ? 0.5 : 0.3)
  }

}