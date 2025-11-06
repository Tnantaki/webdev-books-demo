import clsx, { type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...args: ClassValue[]) {
	return twMerge(clsx(args));
}

const monthNames = [
	"Jan", "Feb", "Mar", "Apr", "May", "Jun", 
	"Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];

export function getThailandDatetime(date: string) {
	const dateObj = new Date(date);
	// Calculate the offset for +7 hours in milliseconds
	const gmtPlus7OffsetMs = 7 * 60 * 60 * 1000;
	const dateThaiZone = new Date(dateObj.getTime() + gmtPlus7OffsetMs);
	
	const day = String(dateThaiZone.getUTCDate()).padStart(2, '0');
	const month = monthNames[dateThaiZone.getUTCMonth()];
	const year = dateThaiZone.getUTCFullYear();
	
	const formattedDate = `${day} ${month} ${year}`;
	
	const hours = String(dateThaiZone.getUTCHours()).padStart(2, '0');
	const minutes = String(dateThaiZone.getUTCMinutes()).padStart(2, '0');
	const seconds = String(dateThaiZone.getUTCSeconds()).padStart(2, '0');
	
	const formattedTime = `${hours}:${minutes}:${seconds}`;
	return {
		date: formattedDate,
		time: formattedTime
	}
}