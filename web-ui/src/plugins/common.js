import Toolbar from '../components/Toolbar';
import ContainerView from '../components/ContainerView';
import MessageBox from '../components/MessageBox';
import DeleteDialog from '../components/DeleteDialog';
import Model from '../common/model/Model';

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

		Vue.prototype.Model = Model;
	}
}

export default Common;
