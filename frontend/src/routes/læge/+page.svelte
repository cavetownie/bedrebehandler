<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '../card.svelte';
  import { Toggle } from 'flowbite-svelte';

  let json: any[] = [];

  let findOpen: boolean = false;

  async function populate() {
    try {
      let url = "http://localhost:8080/behandlere/læge";
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

<div class="flex flex-wrap justify-center gap-4 w-full">
<Toggle bind:checked={findOpen} on:change={() => populate()}>Vis kun behandlere som er åbne nu!</Toggle>
{#each json as behandler, i}
    <Card navn={behandler.navn} adresse={behandler.adresse} postnummer={behandler.postnummer} id={behandler.identifier}>
    </Card>
{/each}
</div>
