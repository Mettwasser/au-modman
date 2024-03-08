<script lang="ts">
	import { Autocomplete, ListBox, ListBoxItem, getModalStore } from '@skeletonlabs/skeleton';
	import { filterList } from '../utils/filterList';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faCheck } from '@fortawesome/free-solid-svg-icons';
	import { icon } from '@fortawesome/fontawesome-svg-core';

	const modalStore = getModalStore();

	// stuff to return
	let profileName: string = $modalStore[0].meta?.profileName ?? '';
	let selectedMods: string[] = $modalStore[0].meta?.selectedMods ?? [];
	let confirmed = false;
	$modalStore[0].response!({ profileName, selectedMods, confirmed });

	// vars for searching through the mods
	let filter = '';
	let mods = [
		'The Other Roles',
		'The Town Of Us',
		'Additional Maps',
		'Some Imaginary Mod',
		'Another Imaginary Mod',
		'Another Other Imaginary Mod'
	];
</script>

<!-- 
	HOW THIS IS GONNA WORK:
	Select mods
 -->

{#if $modalStore[0]}
	<!--  w-[650px] -->
	<div class="max-h-[650px] w-[650px] rounded-lg bg-surface-800">
		<div class="flex flex-col justify-start space-y-2 p-8">
			<h2 class="mb-4 ml-4 text-center text-4xl">Add a Profile</h2>
			<div
				class="
                !mb-4 mt-6 flex flex-col flex-wrap items-center gap-6 child:w-2/3 child:space-y-1 [&>div>input]:rounded-lg
                "
			>
				<div>
					<label for="profileName">Profile Name</label>
					<input id="profileName" bind:value={profileName} class="input" type="text" placeholder="Profile Name..." />
				</div>

				<div>
					<h3 class="mb-4 ml-4 text-center text-2xl">Mods</h3>
					<input class="input" type="search" bind:value={filter} placeholder="Search..." />
					<div class="h-full max-h-60 overflow-y-auto">
						<ListBox active="variant-filled-success !text-white" hover="hover:variant-soft-success" multiple class="w-full">
							{#each filterList(mods, filter) as mod (mod)}
								<ListBoxItem bind:group={selectedMods} name="medium" value={mod}>
									<svelte:fragment slot="trail">
										{#if selectedMods.includes(mod)}
											<FontAwesomeIcon icon={faCheck} />
										{/if}
									</svelte:fragment>
									{mod}
								</ListBoxItem>
							{/each}
						</ListBox>
					</div>
				</div>
			</div>
			<div class="mr-5 flex justify-end">
				<button class="variant-filled-success btn !text-white">Create</button>
			</div>
		</div>
	</div>
{/if}
