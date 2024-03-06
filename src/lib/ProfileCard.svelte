<script lang="ts">
	import '@fortawesome/fontawesome-svg-core/styles.css';
	import { faPlay } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	export let name: string;
</script>

<div class="card flex flex-wrap items-center w-1/3">
	<header class="m-4 h6 text-center align-middle">{name}</header>
	<div class="ml-auto mr-5">
		<button type="button" class="btn-icon rounded-md w-8 h-8 btn-icon-md variant-filled-success">
			<FontAwesomeIcon icon={faPlay} color="#FFFFF" />
		</button>
	</div>
</div>
