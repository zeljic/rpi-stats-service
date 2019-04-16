export default {
	bind(el, binding, vnode)
	{
		const me = vnode.context;
		let event = null;

		me.$watch('show', (state) =>
		{
			if (state)
			{
				event = (e) =>
				{
					if (e.key === 'Escape')
					{
						me.$emit('update:show', false);
					}
				};

				document.addEventListener('keydown', event);
			} else if (event !== null)
			{
				document.removeEventListener('keydown', event);

				event = null;
			}
		});
	}
};
