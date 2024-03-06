<script lang="ts">
	import { page } from '$app/stores';
	import { faGear, faGears, faHouse, faUser } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { TabAnchor, TabGroup } from '@skeletonlabs/skeleton';
	import '../app.postcss';
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
		<TabAnchor href="/settings" selected={$page.url.pathname === '/settings'} class="lg:absolute lg:right-8">
			<FontAwesomeIcon icon={faGear} />
		</TabAnchor>
	</TabGroup>
	<div class="h-full child:container p-4 flex justify-center overflow-y-scroll">
		<slot />
	</div>
</div>
