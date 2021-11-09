<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <graph ref="graph" />
    
    <analytic-status />
    <div id="controls">
      <div v-if="analyticUnitType == analyticUnitTypes[1]">
        Hold <pre>S</pre> to label patterns <br/>
        Hold <pre>A</pre> to label anti patterns <br/>
        Holde key <pre>D</pre> to delete patterns
        <br/>
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
    }
  },
  data: function () {
    return {
      analyticUnitTypes: [AnalyticUnitType.THRESHOLD, AnalyticUnitType.PATTERN, AnalyticUnitType.ANOMALY],
    }
  },
  computed: {
    analyticUnitType() {
      return this.$store.state.analyticUnitType;
    }
  }
});
</script>

<style scoped>
pre {
  display: inline;
}
</style>
