<template>
	<container-view>

		<v-layout row wrap>
			<v-flex x12>
				<toolbar>
					Log Type Add
				</toolbar>
			</v-flex>
			<v-flex xs12>
				<v-form ref="frmLogTypes" @submit.prevent="submit">
					<v-card>
						<v-card-text>
							<v-layout>
								<v-flex xs12>
									<v-checkbox
										label="Enabled"
										v-model="item.enabled"
										:disabled="disabled"
										box
									></v-checkbox>
								</v-flex>
							</v-layout>
							<v-layout row wrap>
								<v-flex xs12 md4>
									<v-text-field
										label="Name"
										v-model="item.name"
										:disabled="disabled"
										box
									></v-text-field>
								</v-flex>

								<v-flex xs12 md8>
									<v-text-field
										label="Description"
										v-model="item.description"
										:disabled="disabled"
										box
									></v-text-field>
								</v-flex>
							</v-layout>
						</v-card-text>
						<v-card-actions>
							<v-spacer></v-spacer>
							<v-btn type="submit" color="default" to="/log-types">Cancel</v-btn>
							<v-btn type="submit" color="primary" :disabled="disabled">Apply</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-flex>
		</v-layout>

	</container-view>
</template>

<script>
	import model from '../../common/model/Model';


	export default {
		name: 'log-types-form',
		data()
		{
			return {
				disabled: false,
				item: {
					id: null,
					name: null,
					description: null,
					enabled: false
				},
				valid: false
			};
		},
		computed: {
			mode()
			{
				return this.$route.params.id ? model.mode.UPDATE : model.mode.CREATE;
			}
		},
		created()
		{
			if (this.mode === model.mode.UPDATE)
			{
				this.fetch();
			}
		},
		methods: {
			submit()
			{
				this.$http({
					url: '/api/log-type',
					method: this.mode === model.mode.CREATE ? 'POST' : 'PUT',
					data: {
						...this.item
					}
				});
			},
			fetch()
			{
				this.disabled = true;

				this.$http({
					url: '/api/log-type/' + this.$route.params.id
				}).then(({data: {item}}) =>
				{
					this.item = item;
				}).finally(() =>
				{
					this.disabled = false;
				});
			}
		}
	};
</script>

<style scoped>

</style>
