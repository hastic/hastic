import { HasticPod } from './hastic_pod';
import { TimeRange  } from '@/types';

import { Segment, SegmentId, SegmentType } from "@/types/segment";
import { LineTimeSerie } from '@chartwerk/line-pod';
import { SegmentsSet } from '@/types/segment_set';


export type UpdateDataCallback = (range: TimeRange) => Promise<{
  timeserie: LineTimeSerie[],
  segments: Segment[]
}>;
export type CreateSegmentCallback = (segment: Segment) => Promise<SegmentId>;
export type DeleteSegmentCallback = (from: number, to: number) => Promise<number>;


export class PatternPod extends HasticPod<UpdateDataCallback> {

  private _udc: UpdateDataCallback;
  private _csc: CreateSegmentCallback;
  private _dsc: DeleteSegmentCallback;

  private _aKeyIsDown: boolean;
  private _sKeyIsDown: boolean;
  private _dKeyIsDown: boolean;
  private _labelBrush: boolean;
  private _antiLabelBrush: boolean;
  private _deleteBrush: boolean;

  constructor(
    el: HTMLElement,
    udc: UpdateDataCallback,
    csc: CreateSegmentCallback,
    dsc: DeleteSegmentCallback,
    segmentSet: SegmentsSet<Segment>
  ) {
    super(el, udc, segmentSet)

    this._csc = csc;
    this._dsc = dsc;

    this._sKeyIsDown = false;
    this._aKeyIsDown = false;
    this._dKeyIsDown = false;

    this._labelBrush = false;
    this._antiLabelBrush = false;

    window.addEventListener("keydown", e => {
      if(e.code == "KeyA") {
        this._aKeyIsDown = true;
      }
      if(e.code == "KeyS") {
        this._sKeyIsDown = true;
      }
      if(e.code == 'KeyD') {
        this._dKeyIsDown = true;
      }
    });
    window.addEventListener("keyup", (e) => {
      if(e.code == "KeyA") {
        this._aKeyIsDown = false;
      }
      if(e.code == "KeyS") {
        this._sKeyIsDown = false;
      }
      if(e.code == 'KeyD') {
        this._dKeyIsDown = false;
      }
    });


    this.fetchData();

  }


  public fetchData(): void {
    let to = Math.floor(Date.now() / 1000);
    let from = to - 50000; // -50000 seconds

    if(!(this.state.xValueRange[0] == 0 && this.state.xValueRange[1] == 1)) {
      [from, to] = this.state?.xValueRange;
      console.log('took from range from state');
    } else {
      console.log('took from range from default');
    }
    console.log(from + " ---- " + to);

    this.udc({ from, to })
      .then(resp => {
        this.updateSegments(resp.segments);
        this.updateData(resp.timeserie, undefined, true);
      })
      .catch(() => { /* set "error" message */ })
  }


  protected onBrushStart(): void {
    if(this._sKeyIsDown) {
      this._labelBrush = true;
      this.svg.select('.selection')
        .attr('fill', 'red');
    } else if (this._aKeyIsDown) {
      this._antiLabelBrush = true;
      this.svg.select('.selection')
        .attr('fill', 'blue');
    } else if (this._dKeyIsDown) {
      this._deleteBrush = true;
      this.svg.select('.selection')
        .attr('fill', 'darkgreen');
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
    if(!this._labelBrush && !this._antiLabelBrush && !this._deleteBrush) {
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

      if(this._labelBrush) {
        this.addSegment(startTimestamp, endTimestamp, SegmentType.LABEL);
        this._labelBrush = false;
      }
      if(this._antiLabelBrush) {
        this.addSegment(startTimestamp, endTimestamp, SegmentType.ANTI_LABEL);
        this._antiLabelBrush = false;
      }
      if(this._deleteBrush) {
        this.deleteSegment(startTimestamp, endTimestamp);
        this._deleteBrush = false;
      }
    }
  }


  protected async addSegment(from: number, to: number, type: SegmentType): Promise<void> {
    const id = this.getNewTempSegmentId();
    from = Math.floor(from);
    to = Math.ceil(to);

    if (from > to) {
      const t = from;
      from = to;
      to = t;
    }

    const segment = new Segment(id, from, to, type);
    //const storedId =
    await this._csc(segment);
    this.fetchData();
    // segment.id = storedId;

    // this._segmentSet.addSegment(segment);
    // this.renderSegment(segment);
  }

  protected async deleteSegment(from: number, to: number): Promise<void> {
    from = Math.floor(from);
    to = Math.ceil(to);

    if (from > to) {
      const t = from;
      from = to;
      to = t;
    }

    await this._dsc(from, to);
    this.fetchData();

  }


  protected updateSegments(segments: Segment[]): void {
    this.segmentSet.clear();
    this.segmentSet.setSegments(segments);
  }

  // TODO: move to "controller"
  private _tempIdCounted = -1;
  public getNewTempSegmentId(): SegmentId {
    this._tempIdCounted--;
    return this._tempIdCounted.toString();
  }

}