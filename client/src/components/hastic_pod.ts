import { ChartwerkLineChart, LineOptions, LineTimeSerie } from "@chartwerk/line-pod";

export class HasticPod extends ChartwerkLineChart {
  constructor(el: HTMLElement, series?: LineTimeSerie[], options?: LineOptions) {
    super(el, series, options)
  }

}