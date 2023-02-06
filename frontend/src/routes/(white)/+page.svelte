<script lang="ts">
	import LinkButton from '../../components/LinkButton.svelte';

	import ExternalLink from '../../components/external-link.svelte';
</script>

<div class="p-1">
	<ul class="list-none">
		<li class="p-1">
			<LinkButton href="/dashboard">Dashboard</LinkButton>
		</li>
		<li class="p-1">
			<LinkButton href="/queue">Queue</LinkButton>
		</li>
		<li class="p-1">
			<LinkButton href="https://www.tokiodreamy.com/">
				<span class="flex items-center">
					Shop&nbsp<ExternalLink />
				</span>
			</LinkButton>
		</li>
	</ul>
</div>
