module.exports = {
	configureWebpack: config => {
		config.resolve.alias['vue$'] = 'vue/dist/vue.esm.js';
	},

	productionSourceMap: false
};
