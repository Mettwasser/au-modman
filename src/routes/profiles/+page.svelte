<script lang="ts">
	import ProfileCard from '$lib/ProfileCard.svelte';
	let names = ['P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8', 'P9', 'P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8'];
	// let names = ['P1', 'P2', 'P3', 'P4', 'P5', 'P6'];
</script>

<div class="child:m-4 flex flex-wrap child:basis-1/3 justify-center overflow-y-scroll">
	{#each names as name}
		<ProfileCard {name} />
	{/each}
</div>
