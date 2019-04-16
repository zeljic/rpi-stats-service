<template>
	<v-dialog v-close-on-esc lazy persistent v-model="show" :width="width">
		<v-card>
			<v-card-title class="title mb-0">{{title}}</v-card-title>
			<v-progress-linear
				color="primary"
				height="2"
				indeterminate
				class="ma-0"
				v-show="loader"
			></v-progress-linear>
			<v-card-text>
				{{message}}
			</v-card-text>
			<v-card-actions>
				<slot name="actions"></slot>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>

<script>
	import CloseOnEsc from '../common/close-on-escape';

	export default {
		data()
		{
			return {
				event: null
			};
		},
		props: {
			show: {
				type: Boolean,
				'default': false
			},
			title: String,
			message: String,
			width: {
				type: Number,
				'default': 480
			},
			loader: {
				type: Boolean,
				'default': false
			}
		},
		directives: {
			'close-on-esc': CloseOnEsc
		}
	};
</script>
