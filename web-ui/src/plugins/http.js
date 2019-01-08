import axios from 'axios';

class Http
{
	constructor(Vue, store)
	{
		this.$vue = new Vue();
		this.$store = store;
	}

	install(Vue)
	{
		Vue.prototype.$http = (conf) =>
		{
			const token = this.$store.getters['user.token'];

			if (token)
				axios.defaults.headers.common['X-Token'] = token;

			return axios(conf);
		};
	}
}

export default Http;
