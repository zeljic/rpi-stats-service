<template>
	<v-container grid-list-md :fluid="fluid">
		<slot></slot>
	</v-container>
</template>

<script>
	import {mapGetters} from 'vuex';

	export default {
		name: '',
		data()
		{
			return {};
		},
		computed: {
			...mapGetters({
				fluid: 'ui/fluid'
			})
		},
		created()
		{
		},
		methods: {}
	};
</script>
