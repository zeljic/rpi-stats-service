<template>
	<v-menu lazy offset-y transition="slide-x-transition">
		<template v-slot:activator="{ on }">
			<v-btn small icon flat v-on="on">
				<v-icon small>fas fa-bars</v-icon>
			</v-btn>
		</template>
		<slot></slot>
	</v-menu>
</template>

<script>
	export default {
		name: 'table-bars-button',
		data()
		{
			return {};
		}
	};
</script>
