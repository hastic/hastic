<template>
  <div id="chart"></div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { HasticPod, TimeRange } from "./hastic_pod";
import { getMetrics } from '../services/metrics.service';
import { LineTimeSerie } from "@chartwerk/line-pod";

import _ from "lodash";


async function resolveData(range: TimeRange): Promise<LineTimeSerie[]> {
  const endTime = Math.floor(range.to);
  const startTime = Math.floor(range.from);

  const step = Math.max(Math.round((endTime - startTime) / 5000), 1);

  try {
    let [target, values] = await getMetrics(startTime, endTime, step);
    return [
      { target: target, datapoints: values, color: 'green' },
    ];
  } catch (e) {
    this.$notify({
      title: "Error during extracting metric",
      text: e,
      type: 'error'
    });
    console.error(e);
  }
}

export default defineComponent({
  name: 'Graph',
  props: {},
  mounted() {
    // const endTime = Math.floor(Date.now() / 1000);
    // const startTime = endTime - 1000; // 1000 seconds
    var pod = new HasticPod(document.getElementById('chart'), resolveData.bind(this));
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
