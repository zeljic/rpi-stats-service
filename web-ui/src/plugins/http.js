import axios from 'axios';

class Http
{
	install(Vue)
	{
		const token = window.sessionStorage.getItem('lindenio-token');

		if (token)
			axios.defaults.headers.common['X-Token'] = token;

		Vue.prototype.$http = (conf) =>
		{
			return axios(conf);
		};
	}
}

export default Http;
