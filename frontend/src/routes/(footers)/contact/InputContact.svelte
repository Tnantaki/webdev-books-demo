<script lang="ts">
	interface Props {
		title: string;
		type?: string;
		placeholder: string;
		value: string;
	}

	let { title, type = 'text', placeholder, value = $bindable() }: Props = $props();
</script>

<label class="flex flex-col">
	<p class="text-base font-medium leading-normal pb-2">{title}</p>
	<input class="input-contact form-input resize-none h-14" {placeholder} {type} bind:value />
</label>

