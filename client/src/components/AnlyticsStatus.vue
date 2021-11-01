<template>
  <div class="analytic-status">
    analytic status: <strong> {{ status }} </strong>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';


export default defineComponent({
  name: 'AnalyticStatus',
  components: {
  },
  computed: {
    status() {
      return this.$store.state.analyticStatus;
    }
  }
});
</script>
