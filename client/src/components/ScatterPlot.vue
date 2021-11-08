<template>
  <div>
    <h3>MODEL</h3>
    <div id="chart"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ChartwerkScatterPod } from "@chartwerk/scatter-pod";

import _ from "lodash";


export default defineComponent({
  name: 'ScatterPlot',
  props: {},
  mounted() {
    // var pod = new ChartwerkScatterPod(
    //   document.getElementById('chart'),
    //   [
    //       {
    //         target: 'test1',
    //         datapoints: [
    //           [100, -50, 0],
    //           [200, 150, 0],
    //           [100, 160, 1],
    //           [150, 170, 1],
    //           [150, 180, 0],
    //           [150, 250, 1]
    //         ] as [number, number][],
    //         color: 'red',
    //         lineType: 'dashed',
    //         pointType: 'circle'
    //       },
    //       {
    //         target: 'test2',
    //         datapoints: [
    //           [200, 50, 1],
    //           [175, 60, 0],
    //           [150, 70, 1]
    //         ],
    //         color: 'purple',
    //         pointType: 'rectangle',
    //         pointSize: 5,
    //         yOrientation: 'right',
    //       }
    //     ],
    //     {
    //       axis: {
    //         x: {
    //           format: 'numeric',
    //           range: [-100, 300]
    //         },
    //         y: {
    //           invert: true,
    //           range: [-100, 250]
    //         },
    //         y1: {
    //           isActive: true,
    //           range: [0, 250]
    //         }
    //       },
    //       zoomEvents: {
    //         mouse: {
    //           pan: { isActive: false, orientation: 'both', keyEvent: 'main' },
    //           zoom: { isActive: true, keyEvent: 'shift' },
    //         },
    //         scroll: {
    //           pan: { isActive: false },
    //           zoom: { isActive: true, keyEvent: 'main' }
    //         }
    //       },
    //       crosshair: {
    //         orientation: 'both',
    //         color: 'gray'
    //       },
    //       labelFormat: {
    //         yAxis: 'y',
    //         xAxis: 'x'
    //       },
    //       eventsCallbacks: {
    //         zoomOut: () => { pod.render() }
    //       },
    //       margin: { top: 30, right: 30, bottom: 40, left: 30 },
    //       circleView: true,
    //     }
    //   );
  },
  methods: {
    
  }
});
</script>

<style scoped lang="scss">
#chart {
  margin: auto;
  width: 80%;
  height: 350px;
}
</style>
