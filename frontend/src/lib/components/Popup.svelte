<script lang="ts">
	import { Dialog } from 'bits-ui';

	interface Props extends Dialog.RootProps {}

	let { open = $bindable(false), title, description } = $props();
</script>

<Dialog.Root bind:open>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4"
		></Dialog.Overlay>
		<Dialog.Content
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-sm rounded-xl bg-card-light dark:bg-card-dark p-8 shadow-2xl text-center translate-x-[-50%] translate-y-[-50%]"
		>
			<Dialog.Title class="flex flex-col items-center">
				{@render title()}
			</Dialog.Title>
			<Dialog.Description>
				{@render description()}
			</Dialog.Description>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
