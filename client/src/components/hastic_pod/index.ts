import { ChartwerkLineChart, LineOptions, LineTimeSerie } from "@chartwerk/line-pod";

export class HasticPod extends ChartwerkLineChart {
  constructor(el: HTMLElement, series?: LineTimeSerie[], options?: LineOptions) {
    super(el, series, options)
  }

  renderMetrics() {
    super.renderMetrics();
    console.log('render my metrics');
  }

  private _renderSegments() {
    const m = this.metricContainer;
    console.log(m);
  }
}