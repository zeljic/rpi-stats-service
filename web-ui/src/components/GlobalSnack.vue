<template>
	<v-snackbar
		v-model="snack.show"
		:left="snack.left"
		:bottom="snack.bottom"
		:timeout="snack.timeout"
	>{{snack.message}}
	</v-snackbar>
</template>

<script>
	import {mapGetters} from 'vuex';

	export default {
		name: 'global-snack',
		data()
		{
			return {};
		},
		computed: {
			...mapGetters({
				'snack': 'ui/snack'
			})
		}
	};
</script>
