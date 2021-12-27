<template>
  <div class="analytic-status">
    analytic status: <strong> {{ status.message }} </strong>
  </div>
</template>

<script lang="ts">
import { AnalyticStatus } from "@/store";

import { defineComponent } from 'vue';


export default defineComponent({
  name: 'AnalyticStatus',
  components: {
  },
  computed: {
    status(): AnalyticStatus {
      return this.$store.state.analyticStatus;
    }
  }
});
</script>
