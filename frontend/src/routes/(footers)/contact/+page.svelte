<script lang="ts">
	import { Mail, MapPin, Phone } from '@lucide/svelte';
	import { contacts, map, type Contact } from './data';
	import InputContact from './InputContact.svelte';

	let message = $state({ fullname: '', email: '', subject: '', message: '' });
</script>

<main class="flex-1 px-4 sm:px-10 lg:px-20 py-10">
	<div class="max-w-6xl mx-auto">
		<div class="text-center mb-12">
			<h1 class="text-4xl md:text-5xl font-black leading-tight tracking-tighter mb-4">
				Get in Touch
			</h1>
			<p class="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
				We'd love to hear from you. Please fill out the form below or use our contact details to get
				in touch with any questions or feedback.
			</p>
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
			<div class="bg-white dark:bg-secondary-dark p-8 rounded-xl shadow-md">
				<form class="flex flex-col gap-6">
					<InputContact
						title="Full Name"
						placeholder="Enter your full name"
						value={message.fullname}
					/>
					<InputContact
						title="Email Address"
						placeholder="Enter your email address"
						value={message.email}
						type="email"
					/>
					<InputContact
						title="Subject"
						placeholder="Enter the subject of your message"
						value={message.subject}
					/>
					<label class="flex flex-col">
						<p class="text-base font-medium leading-normal pb-2">Your Message</p>
						<textarea
							class="input-contact form-textarea resize-y min-h-[160px]"
							placeholder="Enter your message here..."
							bind:value={message.message}
						></textarea>
					</label>
					<button
						class="flex min-w-[120px] max-w-full cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-6 bg-primary text-white text-base font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors duration-200"
					>
						<span class="truncate">Send Message</span>
					</button>
				</form>
			</div>
			<div class="flex flex-col gap-8">
				<div class="bg-white dark:bg-secondary-dark p-8 rounded-xl shadow-md">
					<h3 class="text-2xl font-bold mb-6">Our Contact Details</h3>
					<div class="space-y-4">
						{#each contacts as contact}
							{@render display_contact(contact)}
						{/each}
					</div>
				</div>
				<div class="rounded-xl overflow-hidden shadow-md">
					<img
						class="w-full h-64 object-cover"
						alt={map.alt}
						data-location={map.locaton}
						src={map.src}
					/>
				</div>
			</div>
		</div>
	</div>
</main>

{#snippet display_contact(contact: Contact)}
	<div class="flex items-center gap-4">
		<div class="w-10 h-10 flex items-center justify-center rounded-full bg-primary/10 text-primary">
			<contact.icon data-alt={contact.title} />
		</div>
		<p class="text-base">{contact.value}</p>
	</div>
{/snippet}
