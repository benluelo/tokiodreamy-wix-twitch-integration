<script lang="ts">
	import { orderCompleted, registerSse } from '../../../components/client';
	import { breaks } from '../../../components/stores';

	import Card from '../../../components/Card.svelte';
	import LineItem from '../../../components/LineItem.svelte';
	import { onMount } from 'svelte';

	let loaded = false;

	onMount(async () => {
		loaded = true;
		registerSse();
	});
</script>

{#if loaded}
	{#if $breaks.ordered_breaks.length === 0}
		<div>no breaks lol</div>
	{:else}
		<div class="max-w-3xl m-auto">
			<div class="flex flex-col gap-y-2">
				{#each $breaks.ordered_breaks as break_}
					<Card>
						<span slot="header">
							<div class="flex">
								{break_.twitch_username}
								<div class="grow" />
								<span class="font-mono">#{break_.order_id}</span>
							</div>
						</span>
						<span slot="content">
							<div class="flex flex-col gap-y-2 p-2">
								{#if break_.order.buyerNote}
									<div class="font-semi-bold">
										Note: {break_.order.buyerNote}
									</div>
								{/if}
								<div class="flex flex-wrap gap-2">
									{#each break_.order.lineItems as lineItem}
										<LineItem {lineItem} />
									{/each}
								</div>
							</div>
						</span>
					</Card>
				{/each}
			</div>
		</div>
	{/if}
{:else}
	<div>Loading...</div>
{/if}
