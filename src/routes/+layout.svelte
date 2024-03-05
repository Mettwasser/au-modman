<script lang="ts">
	import '../app.postcss';
	import { TabGroup, TabAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import { faUser, faHouse, faGears } from '@fortawesome/free-solid-svg-icons';
</script>

<div class="h-full border-4 border-red-700">
	<TabGroup
		justify="justify-center"
		active="variant-filled-primary"
		hover="hover:variant-soft-primary"
		flex="flex-1 lg:flex-none"
		rounded="rounded-md"
		border=""
		class="bg-surface-100-800-token w-full child:p-2"
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
	</TabGroup>
	<slot />
</div>
