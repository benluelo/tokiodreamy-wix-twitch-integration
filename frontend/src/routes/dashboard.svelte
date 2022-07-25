<script>
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { breaks, Client } from '../components/client';
	import EnsureLoggedIn from '../components/EnsureLoggedIn.svelte';

	onMount(() => {
		Client.sse();
	});
</script>

<div class="p-2 bg-slate-500 h-screen w-screen">
	<!-- <EnsureLoggedIn> -->
	{#each get(breaks).ordered_breaks as break_}
		<div class="rounded-md shadow-lg bg-slate-200 text-slate-900">
			<div class="rounded-t-md bg-slate-300 p-2 border-b border-slate-400">
				{break_.twitch_username} <span class="font-mono float-right">#{break_.order_id}</span>
			</div>
			<div class="font-semi-bold p-2">
				{#if break_.order.buyerNote}
					Note: {break_.order.buyerNote}
				{/if}
			</div>
			<div class="flex flex-wrap p-1">
				{#each break_.order.lineItems as lineItem}
					<div class="rounded-md flex-initial m-1 shadow-lg bg-slate-200">
						<div class="rounded-t-md bg-slate-300 flex-nowrap border-b border-slate-400">
							<span class="whitespace-nowrap flex">
								<div class="grow border-r p-2 border-slate-400">
									{lineItem.name}
								</div>
								<span class="pb-2 w-10 self-end text-center">x{lineItem.quantity}</span>
							</span>
						</div>
						<ul class="list-none p-2 border-b border-slate-300">
							{#if lineItem.customTextFields}
								{#each lineItem.customTextFields as customTextField}
									<li>
										{customTextField.title}: {customTextField.value}
									</li>
								{/each}
							{/if}
						</ul>
						<ul class="list-none p-2">
							{#each lineItem.options as option}
								<li>
									{option.option}: {option.selection}
								</li>
							{/each}
						</ul>
					</div>
				{/each}
				<div class="flex-1" />
				<div class="self-end ml-auto">
					<div class="m-1">
						<div class="shadow-lg">
							<button
								class="text-white bg-blue-700 hover:bg-blue-800 rounded-md p-2 focus:ring-4 focus:ring-blue-300 focus:outline-none"
							>
								Complete
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/each}
	<!-- </EnsureLoggedIn> -->
</div>
