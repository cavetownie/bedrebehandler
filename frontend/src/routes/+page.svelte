<script>
	import { NavBrand, Navbar, NavLi, NavUl, NavHamburger, Skeleton, TextPlaceholder, ImagePlaceholder, Search } from "flowbite-svelte";
	import { Button } from 'flowbite-svelte';
	import { Heading, P, Span } from 'flowbite-svelte';
</script>

<div class="relative px-32 mb-4">
	<Heading tag="h2">
		Find din sundhedsbehandler på en <Span underline decorationClass="decoration-4 decoration-red-400 dark:decoration-red-600">bedre måde!</Span>
	</Heading>
	<P>Søg efter din behandler under:</P>
	<div class="relative px-64">
		<Search placeholder="Find behandler [Søg efter klinik type, navn, eller postnummer (F.eks. læge 7000)]">
			<Button>Søg</Button>
		</Search>
	</div>
	<P>Eller sorter efter klinik type under:</P>

	<div class="relative px-32">
		<Button color="light">Læge</Button>
		<Button color="light">Psykoterapeut</Button>
		<Button color="light">Psykolog</Button>
		<Button color="light">Hudlæge</Button>
		<Button color="light">Øjenlæge</Button>
	</div>
</div>
