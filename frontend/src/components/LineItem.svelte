<script lang="ts">
	import type { OrderLineItem } from 'src/generated/OrderLineItem';
	import KeyValueDisplay from './KeyValueDisplay.svelte';

	export let lineItem: OrderLineItem;
</script>

<div class="rounded-[3px] shadow-sm bg-gray-50 w-full xs:w-auto border border-gray-300">
	<div class="rounded-t-[3px] bg-gray-100 flex-nowrap border-b border-gray-300">
		<span class="whitespace-nowrap flex">
			<div class="grow border-r p-2 border-gray-300">
				{lineItem.name}
			</div>
			<span class="font-mono pb-2 w-10 self-end text-center">x{lineItem.quantity}</span>
		</span>
	</div>
	{#if lineItem.notes}
		<div class="p-2 border-b border-gray-300">
			{lineItem.notes}
		</div>
	{/if}
	<ul class="list-none p-2 border-b border-gray-300">
		{#if lineItem.customTextFields}
			<KeyValueDisplay key="title" value="value" obj={lineItem.customTextFields} />
		{/if}
	</ul>
	<ul class="list-none p-2">
		<KeyValueDisplay key="option" value="selection" obj={lineItem.options} />
	</ul>
</div>
