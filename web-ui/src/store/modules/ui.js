export default {
	namespaced: true,
	state: {
		fluid: false
	},
	mutations: {
		fluid(state, v)
		{
			state.fluid = v;
		}
	},
	actions: {},
	getters: {
		fluid(state)
		{
			return state.fluid;
		}
	}
};
