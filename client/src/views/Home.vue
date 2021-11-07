<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <graph ref="graph" />
    
    <div id="controls">
      <div>
        Hold <pre>S</pre> to label patterns <br/>
        Hold <pre>A</pre> to label anti patterns <br/>
        Holde key <pre>D</pre> to delete patterns
      </div>
      <analytic-status />

      <button @click="clearAllLabeling"> clear all labeling </button>
    </div>

  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Graph from '@/components/Graph.vue';
import AnalyticStatus from '@/components/AnlyticsStatus.vue';


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
  }
});
</script>

<style scoped>
pre {
  display: inline;
}
</style>
