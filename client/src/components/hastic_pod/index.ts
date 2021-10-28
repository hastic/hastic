import { AxisRange } from "@chartwerk/core/dist/types";
import { LinePod, LineTimeSerie } from "@chartwerk/line-pod";

export type TimeRange = { from: number, to: number };
export type UpdateDataCallback = (range: TimeRange) => Promise<LineTimeSerie[]>;

export class HasticPod extends LinePod {

  private _udc: UpdateDataCallback;

  constructor(el: HTMLElement, udc: UpdateDataCallback) {
    super(el, undefined, {
      renderLegend: false,
      eventsCallbacks: {
        zoomIn: range => { this._zoomIn(range) },
        zoomOut: ({x, y}) => { this._zoomOut({x, y}) }
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

  private async _zoomIn(range: AxisRange[]) {
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
}