<script>
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte';
</script>

<div class="flex flex-wrap relative mb-14">
  <Navbar>
    <NavUl class="ml-auto">
      <NavLi href="/" active={true}>Tilbage til start</NavLi>
      <NavLi href="/læge">Læger</NavLi>
      <NavLi href="/tandlæge">Tandlæger</NavLi>
      <NavLi href="/psykolog">Psykologer</NavLi>
    </NavUl>
  </Navbar>
</div>