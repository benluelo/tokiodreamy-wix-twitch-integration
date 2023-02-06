<script lang="ts">
	export let key: string;
	export let value: string;

	export let obj: {
		[x: string]: any;
	}[];
</script>

<dl>
	{#each obj as kv}
		<dt>
			{kv[key]}
		</dt>
		<dd>
			{kv[value]}
		</dd>
	{/each}
</dl>

<style lang="css">
	dl {
		display: grid;
		grid-template-columns: auto minmax(0, 1fr);
	}

	dt,
	dl {
		margin-top: 0.125rem;
	}

	dt {
		font-weight: 600;
		margin-right: 1em;
	}

	dt:first-child,
	dt:first-child + dd {
		margin-top: 0;
	}

	dd {
		margin-left: 0;
	}
</style>
