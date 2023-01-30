<script>
	import { registerSse } from '../../components/client';
	import LineItem from '../../components/LineItem.svelte';
	import { breaks } from '../../components/stores';

	import EnsureLoggedIn from '../../components/EnsureLoggedIn.svelte';
</script>

<EnsureLoggedIn onLoggedIn={registerSse}>
	{#if $breaks.ordered_breaks.length === 0}
		<div>no breaks lol</div>
	{:else}
		<div>
			<div class="flex flex-col max-w-md text-xl">
				{#each $breaks.ordered_breaks as break_, idx}
					{#if idx !== 0}
						<hr />
					{/if}
					<div class={idx === 0 ? 'text-xl' : 'text-lg'}>
						{#if idx === 0}
							<div>Now Opening:</div>
							<hr />
						{:else if idx === 1}
							<div>Up next</div>
							<hr />
						{/if}
						<div class="">
							{break_.twitch_username}
						</div>
						<div class="flex flex-col pl-2 gap-y-1">
							<div class="">
								{#each break_.order.lineItems as lineItem}
									<span class="whitespace-nowrap flex">
										{lineItem.name}
										&nbsp<span class="font-mono">x{lineItem.quantity}</span>
									</span>
								{/each}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</EnsureLoggedIn>

<style>
	hr {
		border-color: black;
	}
</style>