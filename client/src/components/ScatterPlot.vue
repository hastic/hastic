<template>
  <div>
    <h3>MODEL</h3>
    <div id="chart"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ChartwerkScatterPod from "@chartwerk/scatter-pod";

import _ from "lodash";



export default defineComponent({
  name: 'ScatterPlot',
  props: {},
  mounted() {
    
  },
  methods: {
    
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
