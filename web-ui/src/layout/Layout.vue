<template>
	<component :is="component"/>
</template>
<script>
	const Application = () => import(/* webpackChunkName: "application" */ './Application.vue');
	const Login = () => import(/* webpackChunkName: "login" */ './Login.vue');
	const Blank = () => import(/* webpackChunkName: "blank" */ './Blank.vue');

	let unwatch = null;

	export default {
		name: 'layout',
		components: {Blank, Application, Login},
		data()
		{
			return {
				check: false,
				status: false
			};
		},
		computed: {
			component()
			{
				if (this.status && this.check)
					return 'application';
				else if (!this.status && this.check)
					return 'login';
				else
					return 'blank';
			}
		},
		mounted()
		{
			unwatch = this.$store.watch((state, getters) => getters['user/logged'], (v) =>
			{
				this.status = v;
			});

			this.$store.dispatch('user/is', {
				$user: this.$user
			}).then(() =>
			{
				this.check = true;
			});
		},
		destroyed()
		{
			unwatch();
		}
	};
</script>
