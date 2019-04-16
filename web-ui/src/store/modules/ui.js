export default {
	namespaced: true,
	state: {
		fluid: false,
		snack: {
			show: false,
			left: true,
			bottom: true,
			timeout: 0,
			message: undefined,
			count: 0
		}
	},
	mutations: {
		fluid(state, v)
		{
			state.fluid = v;
		},
		snack(state, v)
		{
			state.snack = v;
		},
		snackCount(state, v)
		{
			state.snack.count = v;
		}
	},
	actions: {
		updateSnack({commit, state}, conf)
		{
			commit('snack', Object.assign(state.snack, conf));
		},
		updateSnackCount({commit, getters}, value)
		{
			commit('snackCount', getters.snackCount + value);
		}
	},
	getters: {
		fluid(state)
		{
			return state.fluid;
		},
		snack(state)
		{
			return state.snack;
		},
		snackCount(state)
		{
			return state.snack.count;
		}
	}
};
