<script lang="ts">
	import { page } from '$app/stores';
	import { TabAnchor, TabGroup } from '@skeletonlabs/skeleton';
	import '../app.postcss';

	import { faGears, faHouse, faUser } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
</script>

<div class="h-full container mx-auto">
	<TabGroup
		justify="justify-center"
		active="variant-filled-primary"
		hover="hover:variant-soft-primary"
		flex="flex-1 lg:flex-none"
		rounded="rounded-md"
		border=""
		class="bg-surface-100-800-token ring-[rgb(250_250_250_/_0.05)] rounded-b-xl w-full child:p-2 child:space-x-2"
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
