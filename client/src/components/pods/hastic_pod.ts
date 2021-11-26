import { LinePod, LineTimeSerie } from "@chartwerk/line-pod";
import { AxisRange } from "@chartwerk/core/dist/types";
import { BrushOrientation } from "@chartwerk/core";
import { SegmentsSet } from "@/types/segment_set";
import { ANALYTIC_UNIT_COLORS } from "@/types/colors"
import { Segment, SegmentId, SegmentType } from "@/types/segment";


export abstract class HasticPod<T> extends LinePod {

  protected segmentsContainer;

  constructor(
    el: HTMLElement,
    protected udc: T,
    protected segmentSet: SegmentsSet<Segment>
  ) {
    super(el, undefined, {
      renderLegend: true,
      zoomEvents: {
        mouse: {
          zoom: {
            isActive: true,
            orientation: BrushOrientation.HORIZONTAL
          }
        }
      },
      eventsCallbacks: {
        zoomIn: range => { this.updateRange(range) },
        zoomOut: ({x, y}) => { this._zoomOut({x, y}) },
        panningEnd: range => { this.updateRange(range) }
      }
    });

    
    this.fetchData();
  }

  renderMetrics() {
    super.renderMetrics();
    this.renderSegments();
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


  protected renderSegments(): void {
    const segments = this.segmentSet.getSegments();
    // TODO: this is a bad hack, don't know why
    if(this.metricContainer == null) {
      return;
    }
    this.segmentsContainer = this.metricContainer
      .insert('g', ':first-child')
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

    if(segment.segmentType == SegmentType.LABEL || segment.segmentType == SegmentType.ANTI_LABEL) {
      r.attr('style', 'stroke:rgb(0,0,0); stroke-width:2')
    }
    if(segment.segmentType == SegmentType.ANTI_LABEL) {
      r.attr('fill', ANALYTIC_UNIT_COLORS[1])
    }
  }

  protected async updateRange(range: AxisRange[]) {
    this.fetchData();
  }

  protected _zoomOut({x, y}): void {
    this.fetchData();
  }

  protected updateSegments(segments: Segment[]): void {
    this.segmentSet.clear();
    this.segmentSet.setSegments(segments);
  }

  abstract fetchData();

}
