<template>
  <div id="chart"></div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { HasticPod } from "./hastic_pod";

// TODO: fetch data from API
// TODO: set height
export default defineComponent({
  name: 'Graph',
  props: {},
  mounted() {
    const startTime = 1590590148;
    const arrayLength = 20;
    const data1: [number, number][] = Array.from({ length: arrayLength }, (el, idx) => [Math.floor(Math.random() * 40), startTime + idx * 10000]);
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
        { target: 'test1', datapoints: data1, color: 'green' },
      ],
      options
    );
    pod.render();
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