import Toolbar from '../components/Toolbar';
import ContainerView from '../components/ContainerView';

class Common
{
	constructor()
	{

	}

	install(Vue)
	{
		Vue.component('toolbar', Toolbar);
		Vue.component('container-view', ContainerView);
	}
}

export default Common;
