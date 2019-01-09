import Vue from 'vue';
import './plugins/vuetify';
import router from './router/router';
import store from './store/store';

import Http from './plugins/http';
import User from './common/User';

import Layout from './layout/Layout.vue';

Vue.config.productionTip = false;

Vue.use(new Http(Vue, store));
Vue.use(new User(Vue, store));

new Vue({
	router,
	store,
	render: h => h(Layout)
}).$mount('#app');
