import Profile from '../views/profile/Profile';
import InstancesTable from '../views/instances/InstancesTable';
import MeshesTable from '../views/meshes/MeshesTable';
import LogTypesTable from '../views/log-types/LogTypesTable';

export default [{
	path: '/profile',
	component: Profile,
	name: 'profile'
}, {
	path: '/instances',
	component: InstancesTable,
	name: 'instances-table-view'
}, {
	path: '/meshes',
	component: MeshesTable,
	name: 'meshes-table-view'
}, {
	path: '/log-types',
	component: LogTypesTable,
	name: 'log-types-table-view'
}];
