<template>
  <div>
    <div id="chart"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent, watch } from 'vue';
import { TimeRange } from "@/types";
import { PatternPod } from "./pods/pattern_pod";
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
async function _deleteSegment(from: number, to: number): Promise<number> {
  try {
    return await deleteSegment(from, to);
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
    var s = new SegmentArray();
    this.pod = new PatternPod(
      document.getElementById('chart'),
      resolveData.bind(this),
      addSegment.bind(this),
      _deleteSegment.bind(this),
      s
    );
    this.pod.render();
  },
  // TODO: it's a hack: listen real events about analytics update and use store
  watch: {
    // TODO: choose pog based on config type
    analyticUnitConfig(newConfig, prevConfig) {
      console.log("CONFIG CHANGED");
      if(prevConfig == null) {
        return;
      }
      console.log(prevConfig);
      console.log(newConfig);
      
      this.rerender();
    }
  },
  methods: {
    // @watch('analyticUnitConfig')
    rerender() {
      this.pod.fetchData();
    },
    async deleteAllSegments() {
      await _deleteSegment.bind(this)(0, Date.now());
      this.rerender();
    }
  },
  computed: {
    analyticUnitConfig() {
      return this.$store.state.analyticUnitConfig;
    }
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
