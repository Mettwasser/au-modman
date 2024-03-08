<script lang="ts">
	import { page } from '$app/stores';
	import { faGear, faGears, faHouse, faUser, faUsers } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { Modal, TabAnchor, TabGroup, type ModalComponent } from '@skeletonlabs/skeleton';
	import '../app.postcss';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import AddProfileModal from '$lib/AddProfileModal.svelte';
	initializeStores();

	// Registry for all Modals
	const modalRegistry: Record<string, ModalComponent> = {
		addProfile: { ref: AddProfileModal }
	};
</script>

<Modal components={modalRegistry} />

<div class="flex h-full flex-col justify-center">
	<TabGroup
		justify="justify-center"
		active="bg-gradient-to-br from-tertiary-500/60 to-secondary-500/60"
		hover="hover:variant-soft-primary"
		flex="flex-1 md:flex-none"
		rounded="rounded-md"
		border=""
		class="mt-4 w-full px-6 md:px-0"
		regionList="gap-1"
	>
		<TabAnchor href="/" selected={$page.url.pathname === '/'}>
			<span>Home</span>
			<FontAwesomeIcon icon={faHouse} class="ml-1" />
		</TabAnchor>

		<TabAnchor href="/profiles" selected={$page.url.pathname === '/profiles'}>
			<span>Profiles</span>
			<FontAwesomeIcon icon={faUser} class="ml-1" />
		</TabAnchor>

		<TabAnchor href="/mods" selected={$page.url.pathname === '/mods'}>
			<span>Mods</span>
			<FontAwesomeIcon icon={faGears} class="ml-1" />
		</TabAnchor>
		<div class="!flex gap-1 md:absolute md:right-4">
			<TabAnchor href="/credits" selected={$page.url.pathname === '/credits'} class="hidden md:block">
				<FontAwesomeIcon icon={faUsers} />
			</TabAnchor>
			<TabAnchor href="/settings" selected={$page.url.pathname === '/settings'} class="hidden md:block">
				<FontAwesomeIcon icon={faGear} />
			</TabAnchor>
		</div>
		<TabAnchor href="/settings" selected={$page.url.pathname === '/settings'} class="block md:hidden">
			<FontAwesomeIcon icon={faGear} />
		</TabAnchor>
	</TabGroup>
	<div class="m-2 flex h-full justify-center overflow-hidden p-4">
		<slot />
	</div>
</div>
