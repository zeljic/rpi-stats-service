<template>
	<v-app>
		<v-navigation-drawer app fixed v-model="drawer">
			<v-list dense>
				<v-list-tile v-for="item in sidebar" :key="item.url" :to="item.url">
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
				<v-btn v-for="item in sidebar" :key="item.url" flat :to="item.url">
					<v-icon left small>{{item.icon}}</v-icon>
					{{item.title}}
				</v-btn>

				<v-menu v-model="profileMenu" offset-y lazy :disabled="!profile">
					<v-btn slot="activator" icon :disabled="!profile">
						<v-icon small flat>fa fa-user</v-icon>
					</v-btn>

					<v-list dense v-if="profile">
						<v-card flat>
							<v-card-text>
								Welcome back,<br/> {{profile.name}}
							</v-card-text>
						</v-card>

						<v-divider></v-divider>

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
			}
		},
		async beforeCreate()
		{
			this.$user.profile();
		},
		methods: {
			logout()
			{
				this.$user.logout().then(() =>
				{
					this.$router.replace('/');
				});
			}
		}
	};
</script>
