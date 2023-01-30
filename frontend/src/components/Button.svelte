<script lang="ts">
	import { toggle_primary_secondary, type PrimaryOrSecondary } from './utils';

	export let type: PrimaryOrSecondary = 'secondary';
	export let disabled: boolean | null | undefined;
	export let onclick: () => void;

	let text_color = toggle_primary_secondary(type, 'text-white', 'text-black');
	let bg_color = toggle_primary_secondary(type, 'bg-blue-800', 'bg-gray-100');
	let disabled_bg_color = toggle_primary_secondary(type, 'bg-blue-400', 'bg-gray-200');
	let disabled_text_color = toggle_primary_secondary(type, 'text-white', 'text-gray-600');
	let hover_color = toggle_primary_secondary(type, 'bg-blue-700', 'bg-gray-300');
	let border = toggle_primary_secondary(type, 'border-none', 'border border-gray-300');
	// disabled:{disabled_bg_color}
	// disabled:{disabled_text_color}
</script>

<button
	class="
        {text_color}
        {bg_color}
        hover:{hover_color}
        {border}
        rounded-[3px]
        p-2
        shadow-lg
        active:shadow-sm
		disabled:shadow-none
    "
	{disabled}
	on:click={onclick}
>
	<slot />
</button>
