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
    const endTime = Math.floor(Date.now() / 1000);
    const startTime = endTime - 1000; // 1000 seconds

    const step = Math.max(Math.round((endTime - startTime) / 5000), 1);

    getMetrics(startTime, endTime, step).then(([target, values]) => {
      
      // const zoomIn = (ranges) => { const range = ranges[0]; options.axis.x.range = range; pod.updateData(undefined, options) }
      // const zoomOut = (ranges) => { console.log('zoomout'); options.axis.x.range = undefined; pod.updateData(undefined, options) }
      
      var pod = new HasticPod(
        document.getElementById('chart'),
        [
          { target: target, datapoints: values, color: 'green' },
        ]
      );
      pod.render();
    }).catch(e => {
      this.$notify({
        title: "Error during extracting metric",
        text: e,
        type: 'error'
      });
      console.error(e);
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
