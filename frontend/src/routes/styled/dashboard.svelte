<script lang="ts">
	import { LoginStatus, loginStatus, orderCompleted, registerSse } from '../components/client';
	import { breaks } from '../components/stores';

	import Card from '../components/Card.svelte';
	import EnsureLoggedIn from '../components/EnsureLoggedIn.svelte';
	import Button from '../components/Button.svelte';
	import LineItem from '../components/LineItem.svelte';
	import { get } from 'svelte/store';

	const moveUp = (idx: number) => {};
	const moveDown = (idx: number) => {};
	const complete = (idx: number) => {
		orderCompleted($breaks.ordered_breaks[idx].order_id);
	};
</script>

<EnsureLoggedIn
	onLoggedIn={() => {
		if (get(loginStatus) !== LoginStatus.Success) {
			throw new Error('NOT LOGGED IN');
		}
		registerSse()
	}}
>
	{#if $breaks.ordered_breaks.length === 0}
		<div>no breaks lol</div>
	{:else}
		<div class="flex flex-col gap-y-2">
			{#each $breaks.ordered_breaks as break_, idx}
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
							<div class="flex gap-x-1.5 items-end">
								<Button disabled={idx === 0} onclick={() => moveUp(idx)}>Up</Button>
								<Button
									disabled={idx === $breaks.ordered_breaks.length - 1}
									onclick={() => moveDown(idx)}
								>
									Down
								</Button>
								<div class="grow" />
								<Button disabled={idx !== 0} onclick={() => complete(idx)} type="primary">
									Complete
								</Button>
							</div>
						</div>
					</span>
				</Card>
			{/each}
		</div>
	{/if}
</EnsureLoggedIn>
