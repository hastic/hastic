<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <graph ref="graph" />
    
    <div id="controls">
      <div>
        Hold <pre>ctrl</pre> to label patterns <br/>
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
      this.$refs.graph.rerender();
    }
  }
});
</script>

<style scoped>
pre {
  display: inline;
}
</style>
