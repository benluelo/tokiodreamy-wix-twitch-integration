<script lang="ts">
	import type { Writable } from 'svelte/store';
	import Button from './Button.svelte';

	export let onclick: () => void;

	export let errorMsg: string | undefined = undefined;

	export let disabled: boolean = false;
	export let username: Writable<string>;
	export let password: Writable<string>;
</script>

<div class="flex justify-center">
	<div class="rounded-[4px] shadow-md bg-gray-50 border border-gray-300 max-w-md flex flex-col">
		<label for="username" class="m-2">Username:</label>
		<input
			name="username"
			type="text"
			class="
				rounded-[3px]
				p-2
				border
				border-gray-300
				active:border-blue-700
				m-2
			"
			bind:value={$username}
		/>
		<label for="password" class="block m-2">Password:</label>
		<input
			type="password"
			name="password"
			class="
				rounded-[3px]
				p-2
				border
				border-gray-300
				active:border-blue-700
				block
				m-2
			"
			bind:value={$password}
		/>
		<div class="m-2">
			<Button {disabled} {onclick}>Login</Button>
		</div>
		{#if errorMsg}
			{errorMsg}
		{/if}
	</div>
</div>
