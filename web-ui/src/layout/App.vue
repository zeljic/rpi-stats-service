<template>
	<v-app>
		<v-navigation-drawer app fixed v-model="drawer">
			<v-list dense>
				<v-list-tile @click.prevent="click">
					<v-list-tile-action>
						<v-icon>fa-xs fa-home</v-icon>
					</v-list-tile-action>
					<v-list-tile-content>
						<v-list-tile-title>Home</v-list-tile-title>
					</v-list-tile-content>
				</v-list-tile>
				<v-list-tile @click.prevent="click">
					<v-list-tile-action>
						<v-icon>fa-xs fa-envelope</v-icon>
					</v-list-tile-action>
					<v-list-tile-content>
						<v-list-tile-title>Contact</v-list-tile-title>
					</v-list-tile-content>
				</v-list-tile>
			</v-list>
		</v-navigation-drawer>
		<v-toolbar app fixed dense class="elevation-0" color="transparent">
			<v-toolbar-side-icon @click.stop="drawer = !drawer"></v-toolbar-side-icon>
			<v-toolbar-title>Linden IO</v-toolbar-title>
		</v-toolbar>
		<v-content>
			<v-container fluid fill-height>
				<router-view></router-view>
			</v-container>
		</v-content>
	</v-app>
</template>

<script>
	export default {
		name: 'App',
		data()
		{
			return {
				drawer: true,
				profile: null
			};
		},
		mounted()
		{
			this.$user.profile();
		},
		methods: {
			click()
			{
				this.user.profile();
			}
		}
	};
</script>
