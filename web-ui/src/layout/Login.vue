<template>
	<v-app>
		<v-content>
			<v-container fluid grid-list-md fill-height>

				<v-layout align-center justify-center>
					<v-flex xs12 sm12 md8 lg6 xl4>
						<v-form @submit.prevent="submit">

							<v-card class="elevation-0" color="transparent">
								<v-card-text color="transparent">
									<v-text-field
										solo
										label="E-Mail"
										:disabled="disabled"
										v-model="email"
										autofocus
									></v-text-field>

									<v-text-field
										solo
										label="Password"
										:disabled="disabled"
										v-model="password"
										type="password"
									></v-text-field>
								</v-card-text>

								<v-card-actions>
									<v-spacer></v-spacer>
									<v-btn type="submit">
										Login
									</v-btn>
								</v-card-actions>
							</v-card>

						</v-form>
					</v-flex>
				</v-layout>

			</v-container>
		</v-content>
	</v-app>
</template>
<script>
	export default {
		name: 'Login',
		data()
		{
			return {
				disabled: false,
				email: null,
				password: null
			};
		},
		methods: {
			submit()
			{
				this.disabled = true;

				this.$user.login(this.email, this.password).then(() =>
				{
					this.disabled = false;
				}).then(() =>
				{
					this.disabled = false;
				});
			}
		}
	};
</script>
