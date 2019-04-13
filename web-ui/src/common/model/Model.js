export default class Model
{
	constructor()
	{

	}

	static get mode()
	{
		return {
			'CREATE': 'CREATE',
			'READ': 'READ',
			'UPDATE': 'UPDATE',
			'DELETE': 'DELETE',
			'VIEW': 'VIEW'
		};
	}
}
