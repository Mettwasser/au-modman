<script lang="ts">
	import '../app.postcss';
	import { TabGroup, TabAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faUser, faHouse, faGears, faGear } from '@fortawesome/free-solid-svg-icons';
</script>

<div class="flex flex-col h-full justify-center">
	<TabGroup
		justify="justify-center"
		active="bg-gradient-to-br variant-gradient-tertiary-secondary"
		hover="hover:variant-soft-primary"
		flex="flex-1 lg:flex-none"
		rounded="rounded-md"
		border=""
		class="child:p-2 w-full"
		regionList="space-x-1"
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
		<TabAnchor href="/settings" selected={$page.url.pathname === '/settings'}>
			<FontAwesomeIcon icon={faGear} class="ml-1" />
		</TabAnchor>
	</TabGroup>
	<div class="h-full child:container p-4 flex justify-center">
		<slot />
	</div>
</div>
