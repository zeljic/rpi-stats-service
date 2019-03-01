<template>
	<v-container grid-list-md>
		<toolbar>
			Instances
			<template slot="actions">
				<v-btn flat>Do it!</v-btn>
			</template>
		</toolbar>
	</v-container>
</template>

<script>

	export default {
		name: 'instances-table-view',
		data()
		{
			return {};
		},
		created()
		{
			this.fetch();
		},
		methods: {
			fetch()
			{
				this.$http({
					url: '/api/mesh',
					method: 'get'
				}).then(() =>
				{

				}).then(() =>
				{

				});
			}
		}
	};
</script>
