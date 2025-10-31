import { Mail, MapPin, Phone } from '@lucide/svelte';

export type Contact = (typeof contacts)[0];
export const contacts = [
	{
		title: 'email',
		value: 'contact@thebookstore.com',
		icon: Mail
	},
	{ title: 'phone', value: '+1 (234) 567-890', icon: Phone },
	{
		title: 'location_on',
		value: '123 Bookstore Lane, Readington, BK 12345',
		icon: MapPin
	}
];

export const map = {
	alt: 'A map showing the location of The Bookstore.',
	locaton: 'Readington',
	src: '/images/contact/map.png'
};
