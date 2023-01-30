<script lang="ts">
	import { toggle_primary_secondary } from './utils';

	export let type: 'primary' | 'secondary' = 'secondary';
	export let href: string | null | undefined = undefined;

	$: text_color = toggle_primary_secondary(type, 'text-white', 'text-black');
	$: bg_color = toggle_primary_secondary(type, 'bg-blue-800', 'bg-gray-200');
	$: hover_color = toggle_primary_secondary(type, 'bg-blue-700', 'bg-gray-200');
	$: border = toggle_primary_secondary(type, 'border-none', 'border border-gray-300');
</script>

<a {href}>
	<div
		class="
			{text_color}
			{bg_color}
			hover:{hover_color}
			{border}
			rounded-[3px]
			p-2
			shadow-lg
			active:shadow-sm
			inline-block
		"
	>
		<slot />
	</div>
</a>

<!-- focus:ring-4
        focus:ring-blue-300
        focus:outline-none -->
