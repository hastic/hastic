<template>
  <div class="analytic-status">
    analytic status: <strong> {{ status }} </strong>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { getStatusGenerator } from '@/services/analytics.service';



export default defineComponent({
  name: 'AnalyticStatus',
  components: {
  },
  data: function name() {
    return {
      status: "loading..."
    }
  },
  mounted: async function() {
    let g = getStatusGenerator();
    for await (const data of g) {
      this.status = data.toLowerCase();
    }
  }
});
</script>
