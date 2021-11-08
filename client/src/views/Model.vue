<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <scatter-plot />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ScatterPlot from '@/components/ScatterPlot.vue';

export default defineComponent({
  name: 'MOdel',
  components: {    
    ScatterPlot
  },
  methods: {
    
  }
});
</script>

<style scoped>
pre {
  display: inline;
}
</style>
