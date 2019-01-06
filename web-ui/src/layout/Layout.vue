<template>
	<component :is="component"/>
</template>
<script>
	const App = () => import(/* webpackChunkName: "application" */ './App.vue');
	const Login = () => import(/* webpackChunkName: "login" */ './Login.vue');
	const Blank = () => import(/* webpackChunkName: "blank" */ './Blank.vue');

	let unwatch = null;

	export default {
		name: 'layout',
		components: {Blank, App, Login},
		data()
		{
			return {
				initialCheck: false,
				userStatus: false
			};
		},
		computed: {
			component()
			{
				if (this.userStatus && this.initialCheck)
					return 'app';
				else if (!this.userStatus && this.initialCheck)
					return 'login';
				else
					return 'blank';
			}
		},
		mounted()
		{
			unwatch = this.$store.watch((state) =>
			{
				return state.user.logged;
			}, (v) =>
			{
				this.userStatus = v;
			});

			this.$user.is().then((response) =>
			{
				this.initialCheck = true;

				this.$store.commit('user.logged', response.data.status && response.data.logged);
			});
		},
		destroyed()
		{
			unwatch();
		}
	};
</script>
