<script lang="ts">
	import {
		LoginStatus,
		loginStatus,
		orderCompleted,
		registerSse,
		updateOrder
	} from '../../../components/client';
	import { breaks } from '../../../components/stores';

	import Card from '../../../components/Card.svelte';
	import EnsureLoggedIn from '../../../components/EnsureLoggedIn.svelte';
	import Button from '../../../components/Button.svelte';
	import LineItem from '../../../components/LineItem.svelte';
	import EditIcon from '../../../components/edit.svelte';
	import { get } from 'svelte/store';

	const moveUp = (idx: number) => {};
	const moveDown = (idx: number) => {};
	const complete = (idx: number) => {
		orderCompleted($breaks.ordered_breaks[idx].order_id);
	};
	const editName = () => {
		updateOrder($breaks.ordered_breaks[editing_name_of_idx!].order_id, {
			Name: $breaks.ordered_breaks[editing_name_of_idx!].twitch_username!
		}).then(() => {
			editing_name_of_idx = null;
		});
	};

	let editing_name_of_idx: number | null = null;
</script>

<EnsureLoggedIn
	onLoggedIn={() => {
		if (get(loginStatus) !== LoginStatus.Success) {
			throw new Error('NOT LOGGED IN');
		}
		registerSse();
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
							<!-- if editing a name, then display the editing menu -->
							{#if editing_name_of_idx === idx}
								<input bind:value={break_.twitch_username} />&nbsp
								<Button type="secondary" disabled={false} onclick={() => editName()}>Update</Button>
								<!-- otherwise, display the twitch username if it exists -->
							{:else}
								<span>
									{#if break_.twitch_username}
										{break_.twitch_username}
									{:else}
										<span class="text-red-500">NO USERNAME PROVIDED</span>
									{/if}
									<button on:click={() => (editing_name_of_idx = idx)}>
										<EditIcon />
									</button>
								</span>
							{/if}
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
