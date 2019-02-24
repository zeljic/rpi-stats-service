import Vue from 'vue';
import './plugins/vuetify';
import router from './router/router';

import './plugins/bus';
import Store from './plugins/store';
import Http from './plugins/http';
import User from './plugins/user';

import Layout from './layout/Layout.vue';

Vue.config.productionTip = false;

Vue.use(new Store(Vue));
Vue.use(new Http(Vue));
Vue.use(new User(Vue));

new Vue({
	router,
	render: h => h(Layout)
}).$mount('#app');
