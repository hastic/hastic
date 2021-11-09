<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <graph ref="graph" />
    
    <analytic-status />
    <div>
      Analytic unit type: <select :value="analyticUnitType" @change="changeAnalyticUnitType">
        <option disabled value="">Please Select</option>
        <option v-bind:key="option" v-for="option in analyticUnitTypes" :value="option">{{option}}</option>
      </select> <br/><br/>
    </div>
    <div id="controls">
      <div v-if="analyticUnitType == analyticUnitTypes[1]">
        Hold <pre>S</pre> to label patterns; 
        Hold <pre>A</pre> to label anti patterns <br/>
        Holde key <pre>D</pre> to delete patterns
        <br/>
        <hr/>
        Correlation score: 
        <input :value="analyticUnitConfig.correlation_score" @change="correlationScoreChange" /> <br/>
        Model score: 
        <input :value="analyticUnitConfig.model_score" @change="modelScoreChange" /> <br/><br/>
        <button @click="clearAllLabeling"> clear all labeling </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Graph from '@/components/Graph.vue';
import AnalyticStatus from '@/components/AnlyticsStatus.vue';
import { AnalyticUnitType } from '@/types/analytic_units';


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
      this.$store.dispatch('patchConfig', { [e.target.value]: true } );
    },
    correlationScoreChange(e) {
      this.$store.dispatch('patchConfig', { Pattern: { correlation_score: e.target.value } });
    },
    modelScoreChange(e) {
      this.$store.dispatch('patchConfig', { Pattern: { model_score: e.target.value } });
    }
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
