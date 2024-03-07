<script lang="ts">
	import '@fortawesome/fontawesome-svg-core/styles.css';
	import { faPlay, faWrench } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	export let name: string;
</script>

<div class="card !bg-surface-800/45 border-[1px] border-white/20 rounded-xl flex items-center">
	<header class="m-4 h6 text-center align-middle">{name}</header>
	<div class="ml-auto mr-5">
		<button type="button" class="btn-icon rounded-md w-8 h-8  btn-icon-md variant-filled-surface mr-1">
			<FontAwesomeIcon icon={faWrench} style="color: white;" />
		</button>
		<button type="button" class="btn-icon rounded-md w-8 h-8 btn-icon-md variant-filled-success">
			<FontAwesomeIcon icon={faPlay} style="color: white;" />
		</button>
	</div>
</div>
