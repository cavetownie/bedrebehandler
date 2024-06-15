<script>
    import { Card } from 'flowbite-svelte';
    export let navn = undefined;
    export let adresse = undefined;
    export let postnummer = undefined;
    /**
    * @type {any[]}
    */
    let json = [];
</script>

<Card class="flex flex-wrap justify-center gap-1 w-full" href="/" vertical size="md">
    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{navn}</h5>
    <h5 class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight"><br>{adresse}<br>{postnummer}</h5>
</Card>