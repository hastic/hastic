import { auth } from "./auth.module";
import { createStore } from 'vuex'
import { getConfig, getStatusGenerator } from "@/services/analytics.service";
import { AnlyticUnitConfig, AnalyticUnitType } from '@/types/analytic_units'
// import { notify } from "@kyvg/vue3-notification";


const SET_ANALYTICS_STATUS = 'SET_ANALYTICS_STATUS';
const SET_DETECTOR_CONFIG = 'SET_DETECTOR_CONFIG';
const _SET_STATUS_GENERATOR = '_SET_STATUS_GENERATOR';


type State = {
  analyticStatus: string,
  analyticUnitType?: AnalyticUnitType,
  analyticUnitConfig?: AnlyticUnitConfig,
  _statusGenerator: AsyncIterableIterator<string>
}

const store = createStore<State>({
  state: {
    analyticStatus: 'loading...',
    analyticUnitType: null,
    analyticUnitConfig: null,
    _statusGenerator: null
  },
  mutations: {
    [SET_ANALYTICS_STATUS](state, status: string) {
      state.analyticStatus = status;
    },
    [SET_DETECTOR_CONFIG](state, { analyticUnitType, analyticUnitConfig }) {      
      state.analyticUnitType = analyticUnitType;
      state.analyticUnitConfig = analyticUnitConfig;
    },
    [_SET_STATUS_GENERATOR](state, generator: AsyncIterableIterator<string>) {
      state._statusGenerator = generator;
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
      commit(_SET_STATUS_GENERATOR, g);
      for await (const data of state._statusGenerator) {
        const st = data.toLocaleLowerCase();
        if(state.analyticStatus.toLocaleLowerCase() != 'ready' && st == 'ready') {
          this.dispatch('fetchConfig');
          // TODO: update segments from here
        }
        // this.status = data.toLowerCase();
        commit(SET_ANALYTICS_STATUS, data);
      }
    },
    async fetchConfig({commit}) {
      const [analyticUnitType, analyticUnitConfig] = await getConfig();
      commit(SET_DETECTOR_CONFIG, { analyticUnitType, analyticUnitConfig });
    }
  },
  modules: {
    auth
  }
})

store.dispatch('_runStatusGenerator');

export default store;
