<template>
	<container-view>

		<v-layout row wrap>
			<v-flex x12>
				<toolbar>
					Instance Add
				</toolbar>
			</v-flex>
			<v-flex xs12>
				<v-form ref="frm" @submit.prevent="submit">
					<v-card>
						<v-card-text>
							<v-layout>
								<v-flex xs12>
									<v-checkbox
										label="Enabled"
										box
										v-model="item.enabled"
										:disabled="disabled"
									></v-checkbox>
								</v-flex>
							</v-layout>
							<v-layout row wrap>
								<v-flex xs12 md4>
									<v-text-field
										label="Name"
										box
										v-model="item.name"
										:disabled="disabled"
									></v-text-field>
								</v-flex>

								<v-flex xs12 md8>
									<v-text-field
										label="Description"
										box
										v-model="item.description"
										:disabled="disabled"
									></v-text-field>
								</v-flex>
							</v-layout>
							<v-layout row wrap>
								<v-flex xs12>
									<v-text-field
										label="UUID"
										box
										:clearable="mode === Model.mode.CREATE"
										readonly
										v-model="item.uuid"
										@click:prepend="generateUUID"
										prepend-icon="fas fa-bolt"
										:disabled="disabled"
									></v-text-field>
								</v-flex>
							</v-layout>
						</v-card-text>
						<v-card-actions>
							<v-spacer></v-spacer>
							<v-btn type="submit" color="default" to="/instances">Cancel</v-btn>
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
		name: 'instances-form',
		data()
		{
			return {
				disabled: false,
				item: {
					id: null,
					uuid: null,
					name: null,
					description: null,
					enabled: false
				},
				valid: false
			};
		},
		computed: {
			id()
			{
				return this.$route.params.id;
			},
			mode()
			{
				return this.id ? model.mode.UPDATE : model.mode.CREATE;
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
				this.disabled = true;

				this.$http({
					url: '/api/instance' + (this.mode === model.mode.CREATE ? '' : '/' + this.id),
					method: this.mode === model.mode.CREATE ? 'POST' : 'PUT',
					data: {
						...this.item
					}
				}).finally(() =>
				{
					this.disabled = false;
				});
			},
			fetch()
			{
				this.disabled = true;

				this.$http({
					url: '/api/instance/' + this.$route.params.id
				}).then(({data: {item}}) =>
				{
					this.item = item;
				}).finally(() =>
				{
					this.disabled = false;
				});
			},
			generateUUID()
			{
				if (this.mode === this.Model.mode.UPDATE)
				{
					return;
				}

				this.disabled = true;

				this.$http({
					url: '/api/instance/generate-uuid'
				}).then(({data: {uuid}}) =>
				{
					this.item.uuid = uuid;
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
