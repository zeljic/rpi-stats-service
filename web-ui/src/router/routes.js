import Profile from '../pages/profile/Profile';
import InstancesTableView from '../pages/instances/InstancesTableView';

export default [{
	path: '/profile',
	component: Profile,
	name: 'profile'
}, {
	path: '/instances',
	component: InstancesTableView,
	name: 'instances-table-view'
}];
