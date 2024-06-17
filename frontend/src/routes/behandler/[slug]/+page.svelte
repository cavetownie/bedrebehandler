<script lang="ts">
    import { marked } from 'marked';
    import type { BehandlerData } from "./+page";
    import { onMount } from 'svelte';
    export let data: BehandlerData;

    let behandler: any = {};
    let beskrivelse: string = "";

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
                    beskrivelse = await marked(behandler.beskrivelse); // TODO: xss sanitize
                }
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        }

        populate(data.name);
    });
</script>

<style>
  .card {
    @apply p-4 bg-white rounded-lg shadow-md;
  }

  .field {
    @apply mb-4;
  }

  .label {
    @apply font-bold;
  }
</style>
  
<div class="container mx-auto mt-8">
  <div class="card">
    <div class="field">
      <div class="label">Identifier:</div>
      <div>{behandler.identifier}</div>
    </div>
    <div class="field">
      <div class="label">Postnummer:</div>
      <div>{behandler.postnummer}</div>
    </div>
    <div class="field">
      <div class="label">Kliniktype:</div>
      <div>{behandler.kliniktype}</div>
    </div>
    <div class="field">
      <div class="label">Navn:</div>
      <div>{behandler.navn}</div>
    </div>
    <div class="field">
      <div class="label">Adresse:</div>
      <div>{behandler.adresse}</div>
    </div>
    <div class="field" class:field-hidden={!behandler.beskrivelse}>
      <div class="label">Beskrivelse:</div>
      <div>{@html beskrivelse}</div>
    </div>
    <div class="field">
      <div class="label">Opdateret:</div>
      <div>{new Date(behandler.opdateret).toLocaleString()}</div>
    </div>
  </div>
</div>
