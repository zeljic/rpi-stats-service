import Toolbar from '../components/Toolbar';
import ContainerView from '../components/ContainerView';
import MessageBox from '../components/MessageBox';
import DeleteDialog from '../components/DeleteDialog';

class Common
{
	constructor()
	{

	}

	install(Vue)
	{
		Vue.component('toolbar', Toolbar);
		Vue.component('container-view', ContainerView);
		Vue.component('message-box', MessageBox);
		Vue.component('delete-dialog', DeleteDialog);
	}
}

export default Common;
