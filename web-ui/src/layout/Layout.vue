<template>
	<component :is="component"/>
</template>
<script>
	const App = () => import(/* webpackChunkName: "layout-app" */ './App.vue');
	const Login = () => import(/* webpackChunkName: "layout-login" */ './Login.vue');
	const Blank = () => import(/* webpackChunkName: "layout-blank" */ './Blank.vue');

	export default {
		components: {Blank, App, Login},
		data()
		{
			return {
				initialCheck: false
			};
		},
		computed: {
			showApp: {
				get()
				{
					return this.initialCheck && this.$store.getters['user.logged'];
				},
				set(v)
				{
					this.$store.commit('userStatusChanged', v);
				}
			},
			showLogin: {
				get()
				{
					return this.initialCheck && !this.showApp;
				},
				set(v)
				{
					this.showApp = !v;
				}
			},
			component()
			{
				if (this.showApp)
					return 'app';
				else if (this.showLogin)
					return 'login';
				else
					return 'blank';
			}
		},
		async mounted()
		{
			return await this.$user.is().then(response =>
			{
				this.showApp = response.data.status && response.data.logged === true;

				this.initialCheck = true;
			});
		}
	};
</script>
