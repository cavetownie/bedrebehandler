<script lang="ts">
    import { marked } from 'marked';
    import type { BehandlerData } from "./+page";
    import { onMount } from 'svelte';
    import { Card, Label } from 'flowbite-svelte';
    import DOMPurify from 'dompurify';

    export let data: BehandlerData;

    let behandler: any = {};

    function capFirstLet(s: string) {
      return s[0].toUpperCase() + s.slice(1);
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
                  behandler.beskrivelse = DOMPurify.sanitize(capFirstLet(await marked(behandler.beskrivelse)));
                } else {
                  behandler.beskrivelse = "Denne klinik har ingen beskrivelse."
                }

                if (behandler.kliniktype) {
                  behandler.kliniktype = capFirstLet(behandler.kliniktype);
                }
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        }

        populate(data.name);
    });
</script>

<div class="flex justify-center h-screen py-12">
  <Card class="w-full max-w-screen-lg p-4 bg-white rounded-lg horizontal">
      <div>
        <h5 class="mb-3 text-3xl font-bold dark:text-white">{behandler.navn}</h5>
      </div>

      <div class="py-5">
        <h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Adresse</h5>
        <h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">{behandler.adresse}, <br>{behandler.postnummer}</h5>
        <h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Beskrivelse</h5>
        <h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">{@html behandler.beskrivelse}</h5>
        <h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Kliniktype</h5>
        <h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">{behandler.kliniktype}</h5>
        <h5 class="mb-2 text-2xl font-bold text-white-900 dark:text-white">Sidst opdateret</h5>
        <h5 class="mb-5 font-normal text-gray-700 dark:text-gray-400 leading-tight">{behandler.opdateret}</h5>
      </div>
  </Card>
</div>