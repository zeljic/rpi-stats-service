import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

export default new Vuex.Store({
	state: {
		user: {
			logged: false,
			profile: null,
			token: null
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
		},
		'user.token'(state, v)
		{
			state.user.token = v;

			return window.sessionStorage.setItem('x-token', v);
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
		},
		'user.token'(state)
		{
			return state.user.token || window.sessionStorage.getItem('x-token');
		}
	}
});
