<template>
  <div>
    <div id="chart"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { HasticPod, TimeRange } from "./hastic_pod";
import { getMetrics } from '../services/metrics.service';
import { getSegments, postSegment, deleteSegment } from '../services/segments.service';
import { LineTimeSerie } from "@chartwerk/line-pod";

import { SegmentArray } from '@/types/segment_array';
import { Segment, SegmentId } from '@/types/segment';

import _ from "lodash";

// TODO: move to store
async function resolveData(range: TimeRange): Promise<{
  timeserie: LineTimeSerie[],
  segments: Segment[]
}> {

  const endTime = Math.floor(range.to);
  const startTime = Math.floor(range.from);

  const step = Math.max(Math.round((endTime - startTime) / 5000), 1);

  try {
    // TODO: request in parallel
    let [target, values] = await getMetrics(startTime, endTime, step);
    let segments = await getSegments(startTime, endTime);
    return {
      timeserie: [{ target: target, datapoints: values, color: 'green' }],
      segments: segments
    }
  } catch (e) {
    this.$notify({
      title: "Error during extracting data",
      text: e,
      type: 'error'
    });
    console.error(e);
  }
}

// TODO: move to store
async function addSegment(segment: Segment): Promise<SegmentId> {
  try {
    const id = await postSegment(segment);
    return id;
  } catch (e) {
    this.$notify({
      title: "Error during saving segment",
      text: e,
      type: 'error'
    });
    console.error(e);
  }
}

// TODO: move to store
async function _deleteSegment(from: number, to: number): Promise<SegmentId> {
  try {
    const id = await deleteSegment(from, to);
    return id;
  } catch (e) {
    this.$notify({
      title: "Error during saving segment",
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
    // TODO: fill segmentArray from service
    var s = new SegmentArray();
    var pod = new HasticPod(
      document.getElementById('chart'),
      resolveData.bind(this),
      addSegment.bind(this),
      _deleteSegment.bind(this),
      s
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
