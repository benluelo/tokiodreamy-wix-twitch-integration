<script lang="ts">
	let open = true;
</script>

<!-- card -->
<div class="rounded-[4px] shadow-md bg-gray-50 border border-gray-300">
	<!-- card header -->
	{#if $$slots.header}
		<button
			class="w-full rounded-t-[4px] bg-gray-100 p-2 border-b border-gray-300"
			on:click={() => (open = !open)}
		>
			<slot name="header" />
		</button>
	{/if}
	<!-- card content -->
	{#if open}
		<slot name="content" />
	{/if}
</div>
