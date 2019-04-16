import _ from '../components/GlobalSnack';

class GlobalSnack
{
	install(Vue)
	{
		const store = new Vue().$store;

		Vue.component('global-snack', _);

		Vue.prototype.snack = (conf) =>
		{
			store.dispatch('ui/updateSnack', conf).then(() => {});
		};

		Vue.prototype.snackLoading = (status) =>
		{
			if (status === undefined)
			{
				status = true;
			}

			store.dispatch('ui/updateSnackCount', status ? 1 : -1).then(() =>
			{
				const count = store.getters['ui/snackCount'];

				if ((status && count > 0) || (!status && count === 0))
				{
					store.dispatch('ui/updateSnack', {
						show: status,
						message: status ? 'Učitavanje, molim sačekajte' : ''
					}).then(() => {});
				}
			});
		};
	}
}

export default GlobalSnack;
