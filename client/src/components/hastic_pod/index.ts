import { LinePod, LineTimeSerie } from "@chartwerk/line-pod";
import { AxisRange } from "@chartwerk/core/dist/types";
import { BrushOrientation } from "@chartwerk/core";
import { SegmentsSet } from "@/types/segment_set";
import { ANALYTIC_UNIT_COLORS } from "@/types/colors"
import { Segment, SegmentId, SegmentType } from "@/types/segment";

export type TimeRange = { from: number, to: number };
export type UpdateDataCallback = (range: TimeRange) => Promise<{
  timeserie: LineTimeSerie[],
  segments: Segment[]
}>;
export type CreateSegmentCallback = (segment: Segment) => Promise<SegmentId>;
export type DeleteSegmentCallback = (from: number, to: number) => Promise<number>;

export class HasticPod extends LinePod {

  private _udc: UpdateDataCallback;
  private _csc: CreateSegmentCallback;
  private _dsc: DeleteSegmentCallback;

  private _ctrlKeyIsDown: boolean;
  private _dKeyIsDown: boolean;
  private _labelBrush: boolean;
  private _deleteBrush: boolean;

  protected segmentsContainer;

  constructor(
    el: HTMLElement,
    udc: UpdateDataCallback,
    csc: CreateSegmentCallback,
    dsc: DeleteSegmentCallback,
    private _segmentSet: SegmentsSet<Segment>
  ) {
    super(el, undefined, {
      renderLegend: false,
      zoomEvents: {
        mouse: {
          zoom: {
            isActive: true,
            orientation: BrushOrientation.HORIZONTAL
          }
        }
      },
      eventsCallbacks: {
        zoomIn: range => { this._updateRange(range) },
        zoomOut: ({x, y}) => { this._zoomOut({x, y}) },
        panningEnd: range => { this._updateRange(range) }
      }
    });

    this._csc = csc;
    this._dsc = dsc;
    this._ctrlKeyIsDown = false;
    this._labelBrush = false;

    window.addEventListener("keydown", e => {
      if(e.code == "ControlLeft") {
        this._ctrlKeyIsDown = true;
      }
      if(e.code == 'KeyD') {
        this._dKeyIsDown = true;
      }
    });
    window.addEventListener("keyup", (e) => {
      if(e.code == "ControlLeft") {
        this._ctrlKeyIsDown = false;
      }
      if(e.code == 'KeyD') {
        this._dKeyIsDown = false;
      }
    });

    this._udc = udc;

    this.fetchData();
  }

  renderMetrics() {
    super.renderMetrics();
    this.renderSegments();
  }

  public fetchData(): void {
    let to = Math.floor(Date.now() / 1000);
    let from = to - 50000; // -50000 seconds

    if(this.state?.xValueRange !== undefined) {
      [from, to] = this.state?.xValueRange;
      console.log('took from range from state');
    } else {
      console.log('took from range from default');
    }
    console.log(from + " ---- " + to);

    this._udc({ from, to })
      .then(resp => {
        this.updateSegments(resp.segments);
        this.updateData(resp.timeserie);
      })
      .catch(() => { /* set "error" message */ })
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
      this._labelBrush = true;
      this.svg.select('.selection')
        .attr('fill', 'red')
    } else if (this._dKeyIsDown) {
      this._deleteBrush = true;
      this.svg.select('.selection')
        .attr('fill', 'blue')
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
    if(!this._labelBrush && !this._deleteBrush) {
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
        this.addSegment(startTimestamp, endTimestamp);
        this._labelBrush = false;
      }
      if(this._deleteBrush) {
        this.deleteSegment(startTimestamp, endTimestamp);
        this._deleteBrush = false;
      }
    }
  }

  protected async addSegment(from: number, to: number): Promise<void> {
    const id = this.getNewTempSegmentId();
    from = Math.floor(from);
    to = Math.ceil(to);

    if (from > to) {
      const t = from;
      from = to;
      to = t;
    }

    const segment = new Segment(id, from, to);
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

  protected renderSegments(): void {
    const segments = this._segmentSet.getSegments();
    // TODO: this is a bad hack, don't know why
    if(this.metricContainer == null) {
      return;
    }
    this.segmentsContainer = this.metricContainer
      .append('g')
      .attr('class', 'segmentsContainer')
    for (const s in segments) {
      this.renderSegment(segments[s]);
    }
  }

  protected renderSegment(segment: Segment): void {
    const x = this.xScale(segment.from);
    const y = 0;
    const w = this.xScale(segment.to) - x;
    const h = this.height

    const r = this.segmentsContainer
      .append('rect')
      .attr('x', x)
      .attr('y', y)
      .attr('width', w)
      .attr('height', h)
      .attr('fill', ANALYTIC_UNIT_COLORS[0])
      .attr('opacity', '0.8')
      .attr('pointer-events', 'none')
    
    if(segment.segmentType == SegmentType.LABEL) {
      r.attr('style', 'stroke:rgb(0,0,0); stroke-width:2')
    }
  }

  private async _updateRange(range: AxisRange[]) {
    // in assumption that range have been changed
    console.log('update range.....');
    console.log(range)
    this.fetchData();
  }

  private _zoomOut({x, y}): void {
    this.fetchData();
  }

  protected updateSegments(segments: Segment[]): void {
    this._segmentSet.clear();
    this._segmentSet.setSegments(segments);
  }


  // TODO: move to "controller"
  private _tempIdCounted = -1;
  public getNewTempSegmentId(): SegmentId {
    this._tempIdCounted--;
    return this._tempIdCounted.toString();
  }
}
