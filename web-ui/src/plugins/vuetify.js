import Vue from 'vue';
import Vuetify from 'vuetify/lib';
import '@fortawesome/fontawesome-free/css/all.css';
import 'vuetify/src/stylus/app.styl';
import '../stylus/main.styl';

Vue.use(Vuetify, {
	iconfont: 'md',
	icons: {
		menu: 'fa-lg fa-bars'
	}
});
