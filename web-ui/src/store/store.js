import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
	state: {
		user: {
			logged: false,
			profile: null
		}
	},
	mutations: {
		'user.logged'(state, v)
		{
			state.user.logged = v;
		},
		'user.profile'(state, v)
		{
			state.user.profile = v;
		}
	},
	actions: {},
	getters: {
		'user.logged'(state)
		{
			return state.user.logged;
		},
		'user.profile'(state)
		{
			return state.user.profile;
		}
	}
});
