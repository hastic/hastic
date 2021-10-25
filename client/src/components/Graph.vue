<template>
  <div id="chart"></div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { HasticPod } from "./hastic_pod";
import { getMetrics } from '../services/metrics.service';

import _ from "lodash";

// TODO: fetch data from API
// TODO: set height
export default defineComponent({
  name: 'Graph',
  props: {},
  mounted() {
    const startTime = 1634172070;
    const endTime = 1635110190;
    const step = Math.round((endTime - startTime) / 5000);
    
    getMetrics(startTime, endTime, step).then((res) => {
      // console.log(res);
      let target = _.keys(res["data"]["data"])[0];
      let values = res["data"]["data"][target].map(([a,b]) => [b,a]);

      
      
      // const zoomIn = (ranges) => { const range = ranges[0]; options.axis.x.range = range; pod.updateData(undefined, options) }
      // const zoomOut = (ranges) => { console.log('zoomout'); options.axis.x.range = undefined; pod.updateData(undefined, options) }
      let options = {
        renderLegend: false, 
        usePanning: false, 
        // axis: {
        //    y: { invert: false, range: [0, 350] },
        //    x: { format: 'time' }
        // },
        // eventsCallbacks: { zoomIn: zoomIn, zoomOut }
      }
      var pod = new HasticPod(
        document.getElementById('chart'),
        [
          { target: target, datapoints: values, color: 'green' },
        ],
        options
      );
      pod.render();
    })
    
    
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
