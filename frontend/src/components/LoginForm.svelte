<script lang="ts">
	import Button from '@smui/button';
	import TextField from '@smui/textfield';
	import type { Writable } from 'svelte/store';

	export let onclick: () => void;

	export let errorMsg: string | undefined = undefined;

	export let disabled: boolean = false;
	export let username: Writable<string>;
	export let password: Writable<string>;
</script>

<TextField invalid={errorMsg !== undefined} bind:value={$username} label={'Username:'} />
<TextField
	invalid={errorMsg !== undefined}
	bind:value={$password}
	label={'Password:'}
	type="password"
/>
<Button {disabled} on:click={onclick}>Login</Button>
{#if errorMsg}
	{errorMsg}
{/if}
