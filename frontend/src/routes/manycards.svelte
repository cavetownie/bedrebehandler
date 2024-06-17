<script lang="ts">
    import { Card, Toggle } from 'flowbite-svelte';
    import { onMount } from 'svelte';

    export let type: string = "";

    let json: any[] = [];
    let findOpen: boolean = false;

    async function populate() {
      try {
        let url = "http://localhost:8080/behandlere" + type;
        if (findOpen) {
          url = url + "?åben=true";
        }

        const response = await fetch(url);
        if (!response.ok) {
          throw new Error('Failed to fetch data');
        }
        json = await response.json();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }

    onMount(() => {
      populate();
    });
</script>


<div class="py-4 flex flex-wrap justify-center">
  <Toggle bind:checked={findOpen} on:change={() => populate()}>Vis kun behandlere som er åbne for tidsbestilling nu!</Toggle>
</div>
<div class="flex flex-wrap justify-center gap-4 w-full">
{#each json as behandler, i}
    <Card class="flex flex-wrap justify-center gap-1 w-full" href="/behandler/{behandler.identifier}" horizontal size="md">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{behandler.navn}</h5>
        <h5 class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight"><br>{behandler.adresse}<br>{behandler.postnummer}</h5>
    </Card>
{/each}
</div>