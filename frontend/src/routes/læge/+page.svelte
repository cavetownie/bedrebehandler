<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '../card.svelte';

  let json: any[] = [];

  onMount(() => {
    async function populate() {
      try {
        const response = await fetch('http://localhost:8080/behandlere/læge');
        if (!response.ok) {
          throw new Error('Failed to fetch data');
        }
        json = await response.json();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }

    populate();
  });
</script>

<div class="flex flex-wrap justify-center gap-4 w-full">
{#each json as behandler, i}
    <Card navn={behandler.navn} adresse={behandler.adresse} postnummer={behandler.postnummer} id={behandler.identifier}>
    </Card>
{/each}
</div>
