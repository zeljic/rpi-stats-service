import axios from 'axios';

class Http
{
	constructor(Vue)
	{
		this.vue = new Vue();
	}

	install(Vue)
	{
		Vue.prototype.$http = (conf) =>
		{
			const token = this.vue.$store.getters['user/token'];

			if (token)
			{
				axios.defaults.headers.common['X-Token'] = token;
			}

			this.vue.snackLoading(true);

			const promise = axios(conf);

			promise.catch(reason => console.log(reason));

			promise.finally(() => this.vue.snackLoading(false));

			return promise;
		};
	}
}

export default Http;
