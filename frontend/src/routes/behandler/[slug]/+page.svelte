<script lang="ts">
	import { marked } from 'marked';
	import type { BehandlerData } from './+page';
	import { onMount } from 'svelte';
	import { Card, Li } from 'flowbite-svelte';
	import DOMPurify from 'dompurify';
	import Navbar from '../../navbar.svelte';

	export let data: BehandlerData;

	let behandler: any = {};
	let aabningstider: any[] = [];
	let telefonnumre: any[] = [];

	function capFirstLet(s: string) {
		return s[0].toUpperCase() + s.slice(1);
	}

	function formatOpeningHours(openingHours: any[]) {
		const weekdays = ['Mandag', 'Tirsdag', 'Onsdag', 'Torsdag', 'Fredag', 'Lørdag', 'Søndag'];

		const formattedHours = openingHours.map((entry) => {
			const dayOfWeek = weekdays[entry.day_of_week - 1];
			const openTime = entry.open_time.slice(0, 5);
			const closeTime = entry.close_time.slice(0, 5);

			return {
				dayOfWeek,
				openTime,
				closeTime,
			};
		});

		return formattedHours;
	}

	function formatPhoneNumbers(phoneNumbers: any[]) {
		const formattedPhoneNumbers = phoneNumbers.map((entry) => {
			const identifier = entry.str_identifier;
			const nummer = entry.telefon_nummer;
			const beskrivelse = entry.beskrivelse;

			return {
				identifier,
				nummer,
				beskrivelse,	
			};
		});

		return formattedPhoneNumbers;
	}

	onMount(() => {
		async function populate(id: string) {
			try {
				const response = await fetch('http://localhost:8080/behandler/' + id);
				if (!response.ok) {
					throw new Error('Failed to fetch data');
				}
				const tmp = await response.json();
				behandler = tmp[0];

				if (behandler.beskrivelse) {
					behandler.beskrivelse = DOMPurify.sanitize(
						capFirstLet(await marked(behandler.beskrivelse))
					);
				} else {
					behandler.beskrivelse = 'Denne klinik har ingen beskrivelse.';
				}

				if (behandler.kliniktype) {
					behandler.kliniktype = capFirstLet(behandler.kliniktype);
				}
			} catch (error) {
				console.error('Error fetching behandler data:', error);
			}
		}

		async function get_aabningstider(id: string) {
			try {
				const response = await fetch('http://localhost:8080/behandler/aabningstider/' + id);
				if (!response.ok) {
					throw new Error('Failed to fetch data');
				}

				const tmp = await response.json();
				if (tmp) {
					aabningstider = formatOpeningHours(tmp);
				}
			} catch (error) {
				console.error('Error fetching aabningstids data:', error);
			}
		}

		async function get_telefonnumre(id: string) {
			try {
				const response = await fetch('http://localhost:8080/behandler/telefonnumre/' + id);
				if (!response.ok) {
					throw new Error('Failed to fetch data');
				}

				const tmp = await response.json();
				if (tmp) {
					telefonnumre = formatPhoneNumbers(tmp);
				}
			} catch (error) {
				console.error('Error fetching aabningstids data:', error);
			}
		}
		
		
		populate(data.name);
		get_aabningstider(data.name);
		get_telefonnumre(data.name);
	});
</script>

<Navbar />
<div class="w-screen flex justify-center items-center py-12">
	<Card class="w-full sm:w-4/5 xl:w-[50rem]">
		<h5 class="mb-3 text-3xl font-bold dark:text-white">{behandler.navn}</h5>

		<h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Adresse</h5>
		<h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">
			{behandler.adresse}, <br />{behandler.postnummer}
		</h5>
		<h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Beskrivelse</h5>
		<h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">
			{@html behandler.beskrivelse}
		</h5>
		<h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Kliniktype</h5>
		<h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">
			{behandler.kliniktype}
		</h5>
		<h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Tidsbestilling</h5>
		{#each aabningstider as tid}
			<Li>
				{tid.dayOfWeek}
				{tid.openTime} - {tid.closeTime}
			</Li>
		{/each}
		<h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Telefonnumre</h5>
		{#each telefonnumre as nummer}
			<Li>
				{nummer.identifier}
				{nummer.nummer} {nummer.beskrivelse}
			</Li>
		{/each}
		<h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Sidst opdateret</h5>
		<h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">
			{new Date(behandler.opdateret).toLocaleString()}
		</h5>
	</Card>
</div>