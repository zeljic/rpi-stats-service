<template>
	<v-app>
		<v-content>
			<v-container fluid grid-list-md fill-height>

				<v-layout align-center justify-center>
					<v-flex xs12 sm10 md8 lg6 xl4>
						<v-form @submit.prevent="submit">

							<v-card>
								<v-card-title class="title">Linden IO</v-card-title>
								<v-card-text>
									<v-text-field
										label="E-Mail"
										:disabled="disabled"
										v-model="email"
										outline
										autofocus
									></v-text-field>

									<v-text-field
										label="Password"
										:disabled="disabled"
										v-model="password"
										outline
										type="password"
									></v-text-field>
								</v-card-text>

								<v-card-actions>
									<v-spacer></v-spacer>
									<v-btn type="reset" :disabled="disabled" color="default">Reset</v-btn>
									<v-btn type="submit" :disabled="disabled" color="primary">Login</v-btn>
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
		name: 'login',
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
				});
			}
		}
	};
</script>
