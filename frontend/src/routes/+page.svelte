<script>
	import { NavBrand, Navbar, NavLi, NavUl, NavHamburger, Skeleton, TextPlaceholder, ImagePlaceholder, Button, Search } from "flowbite-svelte";
</script>

<div class="relative px-8">
	<NavUl>
	<NavLi href="/" active={true}>Find behandler</NavLi>
	<NavLi href="/">?</NavLi>
	<NavLi href="/">?</NavLi>
	<NavLi href="/">?</NavLi>
	<NavLi href="/">?</NavLi>
	</NavUl>
</div>
<div class="relative px-64">
	<Search placeholder="Find behandler">
		<Button>Søg</Button>
	</Search>
</div>
