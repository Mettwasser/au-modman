<script lang="ts">
	import { ListBox, ListBoxItem, getModalStore } from '@skeletonlabs/skeleton';

	const modalStore = getModalStore();

	let name = $modalStore[0].meta?.name ?? '';
	let modUrl = $modalStore[0].meta?.name ?? '';

	let confirmed = false;

	let mods = [
		'Stuff A',
		'Stuff B',
		'Stuff C',
		'Stuff A',
		'Stuff B',
		'Stuff C',
		'Stuff A',
		'Stuff B',
		'Stuff C',
		'Stuff A',
		'Stuff B',
		'Stuff C'
	];
	let selectedMods: string[] = [];

	$modalStore[0].response!({ profileName: name, modUrl: modUrl });
</script>

<!-- 
	HOW THIS IS GONNA WORK:
	Select mods
 -->

{#if $modalStore[0]}
	<!--  w-[650px] -->
	<div class="h-[450] w-[650px] overflow-y-auto rounded-lg bg-surface-800">
		<div class="flex flex-col justify-start space-y-2 p-8">
			<h2 class="mb-4 ml-4 text-4xl">Add a Profile</h2>
			<div
				class="
                !mb-4 mt-6 flex flex-wrap justify-center gap-6 child:basis-45/100 child:space-y-1 [&>div>input]:rounded-lg
                "
			>
				<div>
					<label for="profileName">Profile Name</label>
					<input id="profileName" bind:value={name} class="input" type="text" placeholder="Profile Name..." />
				</div>
				<div>
					<label for="modUrl">Mod Download URL</label>
					<input id="modUrl" class="input" type="text" placeholder="Profile Name..." />
				</div>

				<div>
					<ListBox active="variant-filled-primary" hover="hover:variant-soft-primary" multiple class="w-full">
						{#each mods as mod}
							<ListBoxItem bind:group={selectedMods} name="medium" value={mod}>{mod}</ListBoxItem>
						{/each}
					</ListBox>
				</div>
			</div>
			<div class="mr-5 flex justify-end">
				<button class="variant-filled-success btn !text-white">Create</button>
			</div>
		</div>
	</div>
{/if}
