import { ChartwerkLineChart, LineOptions, LineTimeSerie } from "@chartwerk/line-pod";

export class HasticPod extends ChartwerkLineChart {
  constructor(el: HTMLElement, series?: LineTimeSerie[]) {
    super(el, series, {
      renderLegend: false,
    });
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