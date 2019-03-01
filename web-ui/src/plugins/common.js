import Toolbar from '../components/Toolbar';

class Common {

	constructor()
	{

	}

	install(Vue)
	{
		Vue.component('toolbar', Toolbar);
	}

}

export default Common;
