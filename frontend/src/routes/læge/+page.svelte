<script>
  import { Button } from 'flowbite-svelte';

  /**
	 * @type {any[]}
	 */
  let json = [];

  async function roll() {
    try {
      const response = await fetch('http://localhost:8080/behandlere/tandlæge');
      if (!response.ok) {
        throw new Error('Failed to fetch data');
      }
      json = await response.json();
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  }
</script>

<Button on:click={roll}>Roll the dice</Button>

{#each json as behandler, i}
	<div class="flex flex-wrap justify-center gap-4">
    <h1 class="text-lg">{behandler.navn}</h1>
    </div>
{/each}
