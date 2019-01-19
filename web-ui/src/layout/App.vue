<template>
	<v-app>
		<v-navigation-drawer app fixed v-model="drawer">
			<v-list dense>
				<v-list-tile v-for="item in sidebar" :to="item.url">
					<v-list-tile-action>
						<v-icon>{{item.icon}}</v-icon>
					</v-list-tile-action>
					<v-list-tile-content>
						<v-list-tile-title>{{item.title}}</v-list-tile-title>
					</v-list-tile-content>
				</v-list-tile>
			</v-list>
		</v-navigation-drawer>
		<v-toolbar app fixed dense class="elevation-1">
			<v-toolbar-side-icon @click.stop="drawer = !drawer"></v-toolbar-side-icon>
			<v-toolbar-title>Linden IO</v-toolbar-title>

			<v-spacer></v-spacer>

			<v-toolbar-items>
				<v-btn v-for="item in sidebar" flat :to="item.url">
					<v-icon left small>{{item.icon}}</v-icon>
					{{item.title}}
				</v-btn>

				<v-divider v-if=""></v-divider>

				<v-menu v-model="profileMenu" offset-y v-if="profile">
					<v-btn slot="activator" flat>
						<v-icon small flat left>fa fa-user</v-icon>
						{{profile.name}}
					</v-btn>

					<v-list dense>
						<v-list-tile :to="'/profile'">
							<v-list-tile-title>Profile</v-list-tile-title>
						</v-list-tile>

						<v-list-tile @click="logout">
							<v-list-tile-title>Logout</v-list-tile-title>
						</v-list-tile>
					</v-list>
				</v-menu>
			</v-toolbar-items>
		</v-toolbar>
		<v-content>
			<router-view></router-view>
		</v-content>
	</v-app>
</template>

<script>
	import {mapGetters} from 'vuex';
	import sidebar from './sidebar';

	export default {
		name: 'app',
		data()
		{
			return {
				drawer: true,
				profileMenu: false
			};
		},
		computed: {
			...mapGetters({
				profile: 'user.profile',
				'user.profile': 'user.profile'
			}),
			sidebar()
			{
				return sidebar;
			},
			route()
			{
				return this.$route;
			},
			router()
			{
				return this.$router;
			}
		},
		async beforeCreate()
		{
			this.$user.profile();
		},
		methods: {
			logout()
			{
				this.$user.logout().then(response =>
				{
					this.$router.replace('/');
				});
			}
		}
	};
</script>
