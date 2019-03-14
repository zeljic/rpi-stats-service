<template>
	<container-view>
		<toolbar>
			Log types
		</toolbar>
	</container-view>
</template>

<script>
	export default {
		name: '',
		data()
		{
			return {
				items: []
			};
		},
		created()
		{
			this.$http({
				url: '/api/log-type',
				method: 'get'
			}).then(({status, data}) =>
			{
				if (status === 200)
				{
					this.items = data.list;
				}
			}).finally(() =>
			{
				console.log(arguments);
			});
		},
		methods: {}
	};
</script>

<style scoped>

</style>
