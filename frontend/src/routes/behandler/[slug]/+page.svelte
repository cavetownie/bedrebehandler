<script lang="ts">
    import type { BehandlerData } from "./+page";
    export let data:BehandlerData;

    let json: any[] = [];

    async function populate(id: string) {
      try {
        const response = await fetch('http://localhost:8080/behandler/' + id);
        if (!response.ok) {
          throw new Error('Failed to fetch data');
        }
        json = await response.json();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    }

    populate(data.name)
</script>

{#each json as behandler}
  <h1>{behandler.postnummer} {data.name}</h1>
{/each}
