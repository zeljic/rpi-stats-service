import axios from 'axios';

class Http
{
	constructor(Vue)
	{
		this.$store = new Vue().$store;
	}

	install(Vue)
	{
		Vue.prototype.$http = (conf) =>
		{
			const token = this.$store.getters['user/token'];

			if (token)
			{
				axios.defaults.headers.common['X-Token'] = token;
			}

			console.log('$http sent request');

			const promise = axios(conf);

			promise.finally(() =>
			{
				console.log('$http got response');
			});

			return promise;
		};
	}
}

export default Http;
