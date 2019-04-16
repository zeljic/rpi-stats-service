<template>
	<message-box
		title="Delete?"
		message="Are you sure?"
		:show="show"
		:loader="loader"
	>
		<template v-slot:actions>
			<v-spacer></v-spacer>
			<v-btn flat color="default" :disabled="loader" @click="$emit('update:show', false)">Cancel</v-btn>
			<v-btn flat color="primary" :disabled="loader" @click="perform">Delete</v-btn>
		</template>
	</message-box>
</template>

<script>
	export default {
		name: 'delete-dialog',
		data()
		{
			return {
				loader: false
			};
		},
		props: {
			url: {
				type: String,
				required: true
			},
			show: {
				type: Boolean,
				default: false
			},
			closeOnSuccess: {
				type: Boolean,
				default: true
			},
			closeOnFail: {
				type: Boolean,
				default: true
			}
		},
		methods: {
			perform()
			{
				const me = this;

				me.loader = true;

				this.$http({
					url: me.url,
					method: 'DELETE'
				})
					.then(() =>
					{
						me.$emit('success');

						if (me.closeOnSuccess)
						{
							me.$emit('update:show', false);
						}
					})
					.catch(() =>
					{
						me.$emit('fail', 'Server Error');

						if (me.closeOnFail)
						{
							me.$emit('update:show', false);
						}
					})
					.finally(() =>
					{
						me.loader = false;
					});
			}
		}
	};
</script>
