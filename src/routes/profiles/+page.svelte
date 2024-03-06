<script lang="ts">
	import ProfileCard from '$lib/ProfileCard.svelte';
	let names = ['P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8', 'P9'];
	// let names = ['P1', 'P2', 'P3', 'P4', 'P5', 'P6'];
</script>

<div class="space-y-2 overflow-y-scroll">
	{#each names as name (name)}
		<ProfileCard {name} />
	{/each}
</div>
