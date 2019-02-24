class User
{
	constructor(Vue)
	{
		this.$vue = new Vue();
		this.$http = this.$vue.$http;
		this.$store = this.$vue.$store;
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
				this.$store.commit('user/token', response.data.token);

				this.$store.commit('user/logged', true);
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
			if (response.data.status === true)
			{
				this.$store.commit('user/logged', false);
				this.$store.commit('user/profile', null);
				this.$store.commit('user/token', null);

				this.$store.dispatch('user/cleanup').finally();
			}
		});
	}

	async profile()
	{
		return await this.$http({
			url: '/api/user',
			method: 'get'
		}).then(response =>
		{
			if (response.data.status === true && response.data.user)
			{
				this.$store.commit('user/profile', response.data.user);
			}
		});
	}
}

export default User;
