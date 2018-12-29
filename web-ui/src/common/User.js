const setValue = (key, value) =>
{
	window.sessionStorage.setItem(key, value);
};

const getValue = (key) =>
{
	window.sessionStorage.getItem(key);
};

class User
{
	constructor(Vue, store)
	{
		this.$vue = new Vue();
		this.$http = this.$vue.$http;
		this.$store = store;

		this.tokenKey = 'lindenio-token';
	}

	install(Vue)
	{
		Vue.prototype.$user = this;
	}

	async is()
	{
		return await this.$http({
			url: '/api/auth',
			method: 'get'
		});
	}

	async login(email, password)
	{
		return await this.$http({
			url: '/api/auth',
			method: 'post',
			data: {
				email: email,
				password: password
			}
		}).then(response =>
		{
			if (response.data.status === true && response.data.token)
			{
				setValue(this.tokenKey, response.data.token);

				this.$store.commit('userStatusChanged', true);
			}
		});
	}

	async logout()
	{
		return await this.$http({
			url: '/api/auth/logout',
			method: 'get'
		}).then(response =>
		{
			if (response.data.status)
			{
				window.sessionStorage.removeItem(this.tokenKey);

				this.$store.commit('userStatusChanged', false);
			}
		});
	}
}

export default User;
