<template>
	<v-container grid-list-lg fluid v-if="profile">
		<v-layout>
			<v-flex xs12 md8 lg6 xl4>
				<v-form ref="frmProfile" v-model="frmProfileValid" @submit.prevent="frmProfileSubmit">
					<v-card>
						<v-card-title class="title">Profile</v-card-title>
						<v-card-text>
							<v-text-field
								label="Name"
								v-model="profile.name"
								prepend-icon="fa fa-user fa-sm"
							></v-text-field>
							<v-text-field
								label="E-mail"
								v-model="profile.email"
								prepend-icon="fa fa-envelope fa-sm"
							></v-text-field>
						</v-card-text>
						<v-card-actions>
							<v-spacer></v-spacer>
							<v-btn type="submit" color="primary" :disabled="btnProfileDisabled">Change</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-flex>
		</v-layout>
		<v-layout>
			<v-flex xs12 md8 lg6 xl4>
				<v-form ref="frmChangePassword" v-model="frmChangePasswordValid" @submit.prevent="frmChangePasswordSubmit">
					<v-card>
						<v-card-title class="title">Change Password</v-card-title>
						<v-card-text>
							<v-text-field
								label="Old password"
								type="password"
								prepend-icon="fa fa-key fa-sm"
								v-model="passwords.old"
							></v-text-field>
							<v-text-field
								label="New password"
								type="password"
								prepend-icon="fa fa-key fa-sm"
								v-model="passwords.new"
							></v-text-field>
							<v-text-field
								label="New password again"
								type="password"
								prepend-icon="fa fa-key fa-sm"
								v-model="passwords.newAgain"
							></v-text-field>
						</v-card-text>
						<v-card-actions>
							<v-spacer></v-spacer>
							<v-btn type="submit" color="primary" :disabled="btnChangePasswordDisabled">Change</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-flex>
		</v-layout>
	</v-container>
</template>
<script>
	import {mapGetters} from 'vuex';

	export default {
		data()
		{
			return {
				frmProfileValid: null,
				frmChangePasswordValid: null,
				loadingProfile: false,
				loadingPassword: false,
				passwords: {
					'old': '',
					'new': '',
					'newAgain': ''
				}
			};
		},
		computed: {
			...mapGetters({
				profile: 'user/profile'
			}),
			btnChangePasswordDisabled()
			{
				return this.passwords.old.length === 0 ||
					this.passwords.new.length === 0 ||
					this.passwords.newAgain.length === 0 ||
					this.loadingPassword ||
					this.passwords.new !== this.passwords.newAgain;
			},
			btnProfileDisabled()
			{
				return this.profile.name.length === 0 || this.profile.email.length === 0;
			}
		},
		methods: {
			frmProfileSubmit()
			{
				this.loadingProfile = true;

				this.$http({
					url: '/api/user',
					method: 'post',
					data: this.profile
				}).then(() =>
				{
				}).then(() =>
				{
					this.loadingProfile = false;
				});
			},
			frmChangePasswordSubmit()
			{
				this.loadingPassword = true;

				this.$http({
					url: '/api/user/change-password',
					method: 'post',
					data: this.passwords
				}).then(() =>
				{
				}).then(() =>
				{
					this.loadingPassword = false;
				});
			}
		}
	};
</script>
