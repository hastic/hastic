import { auth } from "./auth.module";
import { createStore } from 'vuex'
import { getStatusGenerator } from "@/services/analytics.service";
import { AnalyticType } from './types'
// import { notify } from "@kyvg/vue3-notification";


const SET_ANALYTICS_STATUS = 'SET_ANALYTICS_STATUS';
const SET_ANALYTICS_TYPE = 'SET_ANALYTICS_TYPE';
const _SET_STATUS_GENERATOR = '_SET_STATUS_GENERATOR';



type State = {
  analyticStatus: string,
  analyticType?: AnalyticType,
  _statusGenerator: AsyncIterableIterator<string>
}

const store = createStore<State>({
  state: {
    analyticStatus: 'loading...',
    analyticType: null,
    _statusGenerator: null
  },
  mutations: {
    [SET_ANALYTICS_STATUS](state, status: string) {
      state.analyticStatus = status;
    },
    [SET_ANALYTICS_TYPE](state, atype: AnalyticType) {
      state.analyticType = atype;
    },
    [_SET_STATUS_GENERATOR](state, generator: AsyncIterableIterator<string>) {
      this._statusGenerator = generator;
    }
  },
  actions: {
    async _runStatusGenerator({commit, state}) {
      // notify({
      //   title: "Authorization",
      //   text: "You have been logged in!",
      // });
      if(state._statusGenerator !== null) {
        return;
      }
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

store.dispatch('_runStatusGenerator');

export default store;
