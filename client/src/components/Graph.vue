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
import { AnalyticUnitType } from '@/types/analytic_units';
import { ThresholdPod } from './pods/threshold_pod';


// TODO: move to store
async function resolveDataPatterns(range: TimeRange): Promise<{
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
// TODO: remove code repetition
async function resolveDataThreshold(range: TimeRange): Promise<{
  timeserie: LineTimeSerie[],
  segments: Segment[]
}> {

  const endTime = Math.floor(range.to);
  const startTime = Math.floor(range.from);

  const step = Math.max(Math.round((endTime - startTime) / 5000), 1);

  try {
    // TODO: request in parallel
    let [target, values] = await getMetrics(startTime, endTime, step);
    let segments = await getSegments(startTime, endTime, false);
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

// TODO: convert to class component
export default defineComponent({
  name: 'Graph',
  props: {},
  mounted() {
    this.rebuildGraph();
  },
  // TODO: it's a hack: listen real events about analytics update and use store
  watch: {
    // TODO: choose pog based on config type
    analyticUnitConfig(newConfig, prevConfig) {
      if(prevConfig == null) {
        return;
      }
      // TODO: remove this hack
      if(!_.isEqual(_.keys(newConfig),_.keys(prevConfig))) {
        return;
      }

      this.rerender();
    },
    analyticUnitType(newType, prevType) {
      this.rebuildGraph();
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
    },
    rebuildGraph() {
      let child = document.getElementById('chart').children[0];
      if(child != undefined) {
        document.getElementById('chart').removeChild(child);
      }
      var sa = new SegmentArray();

      const aut = this.analyticUnitType;
      if(aut == null) {
        return;
      }
      
      if(aut === AnalyticUnitType.PATTERN) {
        this.pod = new PatternPod(
          document.getElementById('chart'),
          resolveDataPatterns.bind(this),
          addSegment.bind(this),
          _deleteSegment.bind(this),
          sa
        );
      }
      if(aut === AnalyticUnitType.THRESHOLD) {
        this.pod = new ThresholdPod(
          document.getElementById('chart'),
          resolveDataThreshold.bind(this),
          sa
        );
      }
      this.pod.render();
    }
  },
  computed: {
    analyticUnitConfig() {
      return this.$store.state.analyticUnitConfig;
    },
    analyticUnitType() {
      return this.$store.state.analyticUnitType;
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
