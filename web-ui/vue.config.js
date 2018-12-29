module.exports = {
	configureWebpack: config => {
		config.resolve.alias['vue$'] = 'vue/dist/vue.esm.js';

		config.devServer = config.devServer || {};

		config.devServer.proxy = {
			'/api': {
				target: 'http://127.0.0.1'
			}
		};
	},

	productionSourceMap: false
};
