<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <graph ref="graph" />

    <analytic-status />

    <template v-if="analyticStatus.available">
      <div>
        Analytic unit type:
        <select :value="analyticUnitType" @change="changeAnalyticUnitType">
          <option disabled value="">Please Select</option>
          <option v-bind:key="option" v-for="option in analyticUnitTypes" :value="option">{{option}}</option>
        </select> <br/><br/>
      </div>
      <div id="controls">
        <div v-if="analyticUnitType == analyticUnitTypes[0]">
          Threshold:
          <input v-model="analyticUnitConfig.threshold" @change="thresholdChange" /> <br/><br/>
        </div>
        <div v-if="analyticUnitType == analyticUnitTypes[1]">
          Hold <pre>S</pre> to label patterns;
          Hold <pre>A</pre> to label anti patterns <br/>
          Hold <pre>D</pre> to delete patterns
          <br/>
          <hr/>
          Correlation score:
          <input v-model="analyticUnitConfig.correlation_score" @change="correlationScoreChange" /> <br/>
          Anti correlation score:
          <input v-model="analyticUnitConfig.anti_correlation_score" @change="antiCorrelationScoreChange" /> <br/>
          Model score:
          <input v-model="analyticUnitConfig.model_score" @change="modelScoreChange" /> <br/>
          Threshold score:
          <input v-model="analyticUnitConfig.threshold_score" @change="thresholdScoreChange" /> <br/><br/>
          <button @click="clearAllLabeling"> clear all labeling </button>
        </div>
        <div v-if="analyticUnitType == analyticUnitTypes[2]">
          Hold <pre>Z</pre> to set seasonality timespan
          <hr/>
          <!-- Alpha:
          <input :value="analyticUnitConfig.alpha" @change="alphaChange" /> <br/> -->
          Confidence:
          <input v-model="analyticUnitConfig.confidence" @change="confidenceChange" /> <br/>
          Seasonality:
          <input v-model="analyticUnitConfig.seasonality" @change="seasonalityChange" /> <br/>
          Seasonality iterations:
          <input v-model="analyticUnitConfig.seasonality_iterations" @change="seasonalityIterationsChange" /> <br/>
          <br/>
        </div>
      </div>
    </template>
  </div>
</template>

<script lang="ts">
import Graph from '@/components/Graph.vue';
import AnalyticStatus from '@/components/AnlyticsStatus.vue';
import { AnalyticUnitType } from '@/types/analytic_units';

import { defineComponent } from 'vue';

import * as _ from 'lodash';


// TODO: move config editig to component
export default defineComponent({
  name: 'Home',
  components: {
    Graph,
    AnalyticStatus
  },
  methods: {
    clearAllLabeling() {
      this.$refs.graph.deleteAllSegments();
    },
    changeAnalyticUnitType(e) {
      this.$store.dispatch('patchConfig', { [e.target.value]: null } );
    },

    // Threshold
    thresholdChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.threshold = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig',  { Threshold: cfg });
    },

    // Pattern
    correlationScoreChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.correlation_score = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig',  { Pattern: cfg });
    },
    antiCorrelationScoreChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.anti_correlation_score = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig',  { Pattern: cfg });
    },
    modelScoreChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.model_score = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig', { Pattern: cfg });
    },
    thresholdScoreChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.threshold_score = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig', { Pattern: cfg });
    },

    // Anomaly
    alphaChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.alpha = _.clamp(parseFloat(e.target.value), 0, 1);
      this.$store.dispatch('patchConfig',  { Anomaly: cfg });
    },
    confidenceChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.confidence = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig',  { Anomaly: cfg });
    },
    seasonalityChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.seasonality = parseFloat(e.target.value);
      this.$store.dispatch('patchConfig',  { Anomaly: cfg });
    },
    seasonalityIterationsChange(e) {
      let cfg = _.clone(this.analyticUnitConfig);
      cfg.seasonality_iterations = Math.ceil(e.target.value);
      this.$store.dispatch('patchConfig',  { Anomaly: cfg });
    },
  },
  data: function () {
    return {
      analyticUnitTypes: [
        AnalyticUnitType.THRESHOLD,
        AnalyticUnitType.PATTERN,
        AnalyticUnitType.ANOMALY,
      ]
    }
  },
  computed: {
    analyticUnitType() {
      return this.$store.state.analyticUnitType;
    },
    analyticUnitConfig() {
      return this.$store.state.analyticUnitConfig;
    },
    analyticStatus() {
      return this.$store.state.analyticStatus;
    }
  }
});
</script>

<style scoped>
pre {
  display: inline;
}

#controls {
  width: 50%;
  margin: auto;
}
</style>
