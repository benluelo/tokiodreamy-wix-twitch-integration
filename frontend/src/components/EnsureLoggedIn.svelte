<script lang="ts">
	import { login, loginStatus, LoginStatus } from './client';
	import { checkUsernameAndPasswordSetInStorage, password, username } from './stores';
	import LoginForm from './LoginForm.svelte';
	import { browser } from '$app/environment';

	export const prerender = false;

	export let onLoggedIn: () => void;

	let onLoggedInAlreadyCalled = false;

	$: if ($loginStatus === LoginStatus.Success) {
		if (!onLoggedInAlreadyCalled) {
			onLoggedIn();
			onLoggedInAlreadyCalled = true;
		}
	}

	let clicked = false;

	function setClicked() {
		console.log('clicked');

		clicked = true;
	}
</script>

{#if browser}
	{#if checkUsernameAndPasswordSetInStorage() && $loginStatus === LoginStatus.Success}
		{#await login()}
			Loading...
		{:then}
			<slot />
		{/await}
	{:else if clicked}
		{#await login()}
			<LoginForm
				disabled
				onclick={() => {
					/* disabled */
				}}
				{username}
				{password}
			/>
		{:then}
			<slot />
		{/await}
	{:else}
		<LoginForm onclick={setClicked} {username} {password} />
	{/if}
{/if}
