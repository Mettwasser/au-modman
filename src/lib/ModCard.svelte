<script lang="ts">
	import '@fortawesome/fontawesome-svg-core/styles.css';
	import { faTrash, faWrench } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { Modification } from '../types/Modification';
	export let mod: Modification;
	export let onDelete: (mod: Modification) => Promise<void>;
	export let onEdit: (mod: Modification) => Promise<void>;
</script>

<div class="card flex w-full items-center rounded-xl border-[1px] border-white/20 !bg-surface-800/45">
	<header class="h6 m-4 text-center align-middle">{mod.name}</header>
	<div class="m-5 flex flex-1 justify-end">
		<div class="flex items-center gap-x-2">
			<p>{mod.version}</p>
			<button type="button" class="btn-icon-md variant-filled-surface btn-icon h-8 w-8 rounded-md" on:click={() => onEdit(mod)}>
				<FontAwesomeIcon icon={faWrench} style="color: white;" />
			</button>
			<button type="button" class="btn-icon-md variant-filled-error btn-icon h-8 w-8 rounded-md" on:click={() => onDelete(mod)}>
				<FontAwesomeIcon icon={faTrash} style="color: white;" />
			</button>
		</div>
	</div>
</div>
