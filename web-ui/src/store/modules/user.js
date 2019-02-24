import Cookies from 'js-cookie';

export default {
	namespaced: true,
	state: {
		logged: false,
		profile: null,
		token: null
	},
	mutations: {
		logged(state, v)
		{
			state.logged = v;
		},
		profile(state, v)
		{
			state.profile = v;
		},
		token(state, v)
		{
			state.token = v;

			if (v)
			{
				Cookies.set('x-token', v);
			} else
			{
				Cookies.remove('x-token');
			}
		}
	},
	actions: {
		async is({commit, getters}, {$user})
		{
			await $user.is().then(response => {
				commit('logged', response.data.status && response.data.logged);
				commit('token', getters.token);
			});
		},
		cleanup({commit})
		{
			commit('logged', false);
			commit('token', null);
			commit('profile', null);
		}
	},
	getters: {
		logged(state)
		{
			return state.logged;
		},
		profile(state)
		{
			return state.profile;
		},
		token(state)
		{
			return state.token || Cookies.get('x-token');
		}
	}
};
