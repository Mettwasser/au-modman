<script lang="ts">
	import ProfileCard from '$lib/ProfileCard.svelte';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

	const modal: ModalSettings = {
		type: 'component',
		component: 'addProfile',

		response: (r: boolean | undefined) => console.log('response:', r)
	};

	const modalStore = getModalStore();
	let names = ['P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8', 'P9', 'P1', 'P2', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8'];
</script>

<div class="flex w-4/5 flex-col items-center">
	<div class="flex w-full justify-end">
		<div class="mr-">
			<button
				type="button"
				on:click={() => modalStore.trigger(modal)}
				class="variant-filled-success btn btn-md rounded-md !text-white"
			>
				Add Profile
			</button>
		</div>
	</div>

	<!-- For spacing -->
	<div class="my-2" />
	<!--  -->

	<div
		class="flex h-full flex-wrap justify-center overflow-y-scroll rounded-l-xl bg-black bg-opacity-20 child:m-4 child:mx-6 child:basis-2/5"
	>
		{#each names as name}
			<ProfileCard {name} />
		{/each}
	</div>
</div>
