<script lang="ts">
	import { browser } from '$app/env';
	import { derived, readable, writable } from 'svelte/store';
	import { client, LoginStatus, password, username } from './client';
	import LoginForm from './LoginForm.svelte';

	export const prerender = false;

	function loginSync() {
		console.log('clicked');

		clicked = true;
		errorMsg = undefined;
		promise = $client.login().then((status) => {
			switch (status) {
				case LoginStatus.Error:
					clicked = false;
					errorMsg = 'Unable to login. Please ensure that your credentials are correct.';
					console.log('bad credentials');

					throw new Error("couldn't log in");

				case LoginStatus.Success:
					console.log('logged in');
			}
		});
	}

	let promise: Promise<void>;
	let clicked = false;
	let errorMsg: string | undefined = undefined;
</script>

{#if clicked}
	{#await promise}
		<LoginForm disabled onclick={loginSync} {username} {password} />
	{:then}
		<slot />
	{:catch error}
		<LoginForm onclick={loginSync} {username} {password} errorMsg={(errorMsg || '') + error} />
	{/await}
{:else}
	<LoginForm onclick={loginSync} {username} {password} {errorMsg} />
{/if}
