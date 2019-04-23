import Profile from '../views/profile/Profile';

import InstancesTable from '../views/instances/InstancesTable';
import InstancesForm from '../views/instances/InstancesForm';

import MeshesTable from '../views/meshes/MeshesTable';

import LogTypesTable from '../views/log-types/LogTypesTable';
import LogTypesForm from '../views/log-types/LogTypesForm';

export default [{
	path: '/profile',
	component: Profile,
	name: 'profile'
}, {
	path: '/instances',
	component: InstancesTable,
	name: 'instances-table-view'
}, {
	path: '/instances/create',
	component: InstancesForm,
	name: 'instances-form-create'
}, {
	path: '/instances/edit/:id',
	component: InstancesForm,
	name: 'instances-form-update'
}, {
	path: '/meshes',
	component: MeshesTable,
	name: 'meshes-table-view'
}, {
	path: '/log-types',
	component: LogTypesTable,
	name: 'log-types-table-view'
}, {
	path: '/log-types/create',
	component: LogTypesForm,
	name: 'log-types-form-create'
}, {
	path: '/log-types/edit/:id',
	component: LogTypesForm,
	name: 'log-types-form-update'
}];
