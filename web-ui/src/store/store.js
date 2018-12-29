import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
	state: {
		user: {
			logged: false
		}
	},
	mutations: {
		userStatusChanged(state, v)
		{
			state.user.logged = v;
		}
	},
	actions: {},
	getters: {
		'user.logged'(state)
		{
			return state.user.logged;
		}
	}
});
