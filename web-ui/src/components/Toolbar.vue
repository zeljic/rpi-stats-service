<template>
	<v-toolbar dense class="elevation-1" color="white">
		<v-toolbar-title>
			<slot></slot>
		</v-toolbar-title>
		<v-spacer></v-spacer>
		<slot name="actions"></slot>
	</v-toolbar>
</template>

<script>
	export default {
		name: 'page-toolbar',
		data()
		{
			return {};
		}
	};
</script>
