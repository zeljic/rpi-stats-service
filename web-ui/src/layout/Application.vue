<template>
	<v-app>
		<v-toolbar app fixed dense class="elevation-1 white">
			<v-toolbar-title>Linden IO</v-toolbar-title>

			<v-spacer></v-spacer>

			<v-toolbar-items>
				<template v-for="item in sidebar">
					<v-btn :key="item.url" flat :to="item.url">
						<v-icon left small>{{item.icon}}</v-icon>
						{{item.title}}
					</v-btn>
					<v-divider vertical :key="item.url + '-divider'"></v-divider>
				</template>

				<v-btn flat icon @click="$store.commit('ui/fluid', !fluid)">
					<v-icon small>fas {{fluid ? 'fa-compress' : 'fa-expand'}}</v-icon>
				</v-btn>

				<v-divider vertical></v-divider>

				<v-menu v-model="profileMenu" offset-y lazy :disabled="!profile">
					<v-btn slot="activator" :disabled="!profile" flat icon>
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
		<global-snack></global-snack>
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
				profileMenu: false
			};
		},
		computed: {
			...mapGetters({
				profile: 'user/profile',
				fluid: 'ui/fluid'
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
