<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<main class="grid h-svh w-svw place-content-center overflow-hidden bg-white font-sans text-black">
	{@render children?.()}
</main>
