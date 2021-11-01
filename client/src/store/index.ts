import { auth } from "./auth.module";
import { createStore } from 'vuex'
import { getStatusGenerator } from "@/services/analytics.service";

const SET_ANALYTICS_STATUS = 'setAnalyticsStatus';

export default createStore({
  state: {
    analyticStatus: 'loading...'
  },
  mutations: {
    [SET_ANALYTICS_STATUS](state, status: string) {
      state.analyticStatus = status;
    }
  },
  actions: {
    async runStatusGenerator({commit}) {
      const g = getStatusGenerator();
      for await (const data of g) {
        // this.status = data.toLowerCase();
        commit(SET_ANALYTICS_STATUS, data);
      }
    }
  },
  modules: {
    auth
  }
})
