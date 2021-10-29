import { AxisRange } from "@chartwerk/core/dist/types";
import { LinePod, LineTimeSerie } from "@chartwerk/line-pod";
import { SegmentsSet } from "@/types/segment_set";
import { Segment, SegmentId } from "@/types/segment";

export type TimeRange = { from: number, to: number };
export type UpdateDataCallback = (range: TimeRange) => Promise<LineTimeSerie[]>;

export class HasticPod extends LinePod {

  private _udc: UpdateDataCallback;
  private _ctrlKeyIsDown: boolean;
  private _ctrlBrush: boolean;

  

  constructor(el: HTMLElement, udc: UpdateDataCallback, private _segmentSet: SegmentsSet<Segment>) {
    super(el, undefined, {
      renderLegend: false,
      eventsCallbacks: {
        zoomIn: range => { this._updateRange(range) },
        zoomOut: ({x, y}) => { this._zoomOut({x, y}) },
        panningEnd: range => { this._updateRange(range) }
      }
    });

    this._ctrlKeyIsDown = false;
    this._ctrlBrush = false;

    window.addEventListener("keydown", e => {
      if(e.code == "ControlLeft") {
        this._ctrlKeyIsDown = true;
      }
    });
    window.addEventListener("keyup", (e) => {
      if(e.code == "ControlLeft") {
        this._ctrlKeyIsDown = false;
      }
    });

    this._udc = udc;

    // TODO: move to params
    const to = Math.floor(Date.now() / 1000);
    const from = to - 5000; // -5000 seconds

    this._udc({ from, to })
      .then(ts => { this.updateData(ts); })
      .catch(() => { /* set "error" message */ })
  }

  renderMetrics() {
    super.renderMetrics();
    console.log('render my metrics');
  }

  protected addEvents(): void {    
    this.initBrush();
    this.initPan();

    this.chartContainer
      .on('mouseover', this.onMouseOver.bind(this))
      .on('mouseout', this.onMouseOut.bind(this))
      .on('mousemove', this.onMouseMove.bind(this))
      .on('dblclick.zoom', this.zoomOut.bind(this));
  }

  protected onBrushStart(): void {
    if(this._ctrlKeyIsDown) {
      this._ctrlBrush = true;
      this.svg.select('.selection').attr('fill', 'red')
    }

    // this.in
    // TODO: move to state
    this.isBrushing === true;
    const selection = this.d3.event.selection;
    if(selection !== null && selection.length > 0) {
      this.brushStartSelection = this.d3.event.selection[0];
    }
    this.onMouseOut();
  }

  protected onBrushEnd(): void {
    if(!this._ctrlBrush) {
      super.onBrushEnd();
    } else {
      const extent = this.d3.event.selection;
      this.isBrushing === false;
      this._ctrlBrush = false;
      if(extent === undefined || extent === null || extent.length < 2) {
        return;
      }
      this.chartContainer
        .call(this.brush.move, null);

      const startTimestamp = this.xScale.invert(extent[0]);
      const endTimestamp = this.xScale.invert(extent[1]);
      this.addLabeling(startTimestamp, endTimestamp);
    }
  }

  protected addLabeling(from: number, to: number) {
    // TODO: implement
    // TODO: persistance of the label
    const id = this.getNewTempSegmentId();
    const segment = new Segment(id, from, to);
    this._segmentSet.addSegment(segment);
    this.renderSegment(segment);
  }

  protected renderSegments() {
    let segments = this._segmentSet.getSegments();
    for (let s in segments) {
      console.log(s);
    }
  }

  protected renderSegment(segment: Segment) {

    const x = this.xScale(segment.from);
    const y = 0;
    const w = this.xScale(segment.to) - x;
    const h = this.height

    const r = this.chartContainer
      .append('rect')
      .attr('x', x)
      .attr('y', y)
      .attr('width', w)
      .attr('height', h)
      .attr('fill', 'red')
      .attr('opacity', '0.8')
  }

  private async _updateRange(range: AxisRange[]) {
    console.log('update range');
    console.log(range);
    
    const ts = await this._udc({ from: range[0][0], to: range[0][1] });
    const options = { axis: { x: { range: range[0] } } };
    this.updateData(ts, options);
  }

  private _zoomOut({x, y}) {
    console.log(`${x} -- ${y}`);
  }

  private _renderSegments() {
    const m = this.metricContainer;
    console.log(m);
  }


  // TODO: move to "controller"
  private _tempIdCounted = -1;
  public getNewTempSegmentId(): SegmentId {
    this._tempIdCounted--;
    return this._tempIdCounted.toString();
  }
}