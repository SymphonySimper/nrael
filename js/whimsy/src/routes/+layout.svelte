<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';

	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{#if page.url.pathname !== '/'}
	<a
		href={resolve('/')}
		aria-label="Home"
		class="fixed top-2 left-2 z-999 rounded-full border border-gray-200 bg-white p-2 transition-transform duration-150 hover:scale-110"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="size-4 fill-none stroke-black stroke-2"
			><path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8" /><path
				d="M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"
			/></svg
		>
	</a>
{/if}

<main class="grid h-svh w-svw place-content-center overflow-hidden bg-white font-sans text-black">
	{@render children?.()}
</main>
