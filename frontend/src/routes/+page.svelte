<script>
	import { Search } from "flowbite-svelte";
	import { Button } from 'flowbite-svelte';
	import { Heading, P, Span } from 'flowbite-svelte';
</script>

<div class="container mx-auto py-12 px-6">
	<Heading tag="h1" class="text-4xl font-bold text-center mb-8">
		Find din behandler på en <Span underline decorationClass="decoration-4 decoration-red-400 dark:decoration-red-600">bedre måde!</Span>
	</Heading>

	<div class="text-center mb-8">
		<P class="text-lg">Søg efter din behandler under:</P>
		<div class="flex justify-center mt-4">
			<Search placeholder="Find behandler">
				<Button class="ml-2">Søg</Button>
			</Search>
		</div>
	</div>

	<P class="text-center text-lg mb-4">Find behandlere baseret på klinik type under:</P>

	<div class="flex flex-wrap justify-center gap-4">
		<Button color="light" class="px-6 py-2" on:click={() => window.location.href="/læge"}>Læge</Button>
		<Button color="light" class="px-6 py-2" on:click={() => window.location.href="/tandlæge"}>Tandlæge</Button>
		<Button color="light" class="px-6 py-2" on:click={() => window.location.href="/psykolog"}>Psykolog</Button>
	</div>
</div>
