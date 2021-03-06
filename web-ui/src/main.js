import Vue from 'vue';
import './plugins/vuetify';
import router from './router/router';

import './plugins/bus';
import Common from './plugins/common';
import Store from './plugins/store';
import Http from './plugins/http';
import User from './plugins/user';

import Layout from './layout/Layout.vue';
import GlobalSnack from './plugins/global-snack';

Vue.config.productionTip = false;

Vue.use(new Common());
Vue.use(new Store(Vue));
Vue.use(new Http(Vue));
Vue.use(new User(Vue));
Vue.use(new GlobalSnack());

new Vue({
	router,
	render: h => h(Layout)
}).$mount('#app');
