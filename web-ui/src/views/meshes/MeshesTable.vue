<template>
	<container-view>

		<v-layout row wrap>
			<v-flex xs12>
				<toolbar>
					Meshes

					<template v-slot:actions>
						<v-btn icon flat to="meshes/create">
							<v-icon small>fas fa-plus</v-icon>
						</v-btn>

						<v-btn icon flat @click="fetch">
							<v-icon small>fas fa-sync</v-icon>
						</v-btn>
					</template>
				</toolbar>
			</v-flex>

			<v-flex xs12>
				<v-data-table
					:headers="headers"
					:items="items"
					:loading="loading"
					class="elevation-1"
					hide-actions
				>
					<template v-slot:items="props">
						<td class="w-10">{{props.item.id}}</td>
						<td class="w-200">{{props.item.name}}</td>
						<td>{{props.item.description}}</td>
						<td class="text-xs-center">
							<v-icon small>fas {{props.item.enabled ? 'fa-check' : 'fa-clock'}}</v-icon>
						</td>
						<td class="w-10 pa-0 text-xs-center">
							<TableBarButton>
								<v-list dense>
									<v-list-tile :to="'/meshes/edit/' + props.item.id">
										<v-list-tile-avatar>
											<v-icon small>fas fa-pencil-alt</v-icon>
										</v-list-tile-avatar>
										<v-list-tile-content>
											<v-list-tile-title>Edit</v-list-tile-title>
										</v-list-tile-content>
									</v-list-tile>

									<v-list-tile @click.stop="showDeleteFn(props.item)">
										<v-list-tile-avatar>
											<v-icon small>fas fa-trash-alt</v-icon>
										</v-list-tile-avatar>
										<v-list-tile-content>
											<v-list-tile-title>Delete</v-list-tile-title>
										</v-list-tile-content>
									</v-list-tile>
								</v-list>
							</TableBarButton>
						</td>
					</template>
				</v-data-table>
			</v-flex>
		</v-layout>

		<delete-dialog
			:show.sync="deleteDialog.show"
			:url="`/api/mesh/${this.deleteDialog.id}`"
			close-on-success
			@success="deleted"
		></delete-dialog>
	</container-view>
</template>

<script>
	import TableBarButton from '../../components/TableBarsButton';

	export default {
		name: 'meshes-table',
		components: {TableBarButton},
		data()
		{
			return {
				loading: false,
				deleteDialog: {
					show: false,
					id: null
				},
				headers: [
					{text: 'ID', value: 'id'},
					{text: 'Name', value: 'name'},
					{text: 'Description', value: 'description'},
					{text: 'Status', value: 'enabled', align: 'center', class: 'w-50', sortable: false},
					{text: '', value: 'id', 'class': 'pa-0 ma-0', sortable: false}
				],
				items: []
			};
		},
		created()
		{
			this.fetch();
		},
		methods: {
			fetch()
			{
				this.loading = false;

				this.items = [];

				this.$http({
					url: '/api/mesh',
					method: 'get'
				}).then(({status, data}) =>
				{
					if (status === 200)
					{
						this.items = data.list;
					}
				}).finally(() => this.loading = false);
			},
			showDeleteFn(item)
			{
				this.deleteDialog.show = true;
				this.deleteDialog.id = item.id;
			},
			deleted()
			{
				this.fetch();
			}
		}
	};
</script>
