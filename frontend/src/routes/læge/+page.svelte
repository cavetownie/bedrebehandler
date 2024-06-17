<script lang="ts">
	import { Heading } from 'flowbite-svelte';
  import ManyCards from '../manycards.svelte';
	import Navbar from '../navbar.svelte';
</script>

<Navbar />
<Heading class="flex justify-center">Læger</Heading>
<ManyCards type="/læge" />

