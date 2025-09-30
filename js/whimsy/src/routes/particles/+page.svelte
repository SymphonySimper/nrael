<script lang="ts">
	import { onMount } from 'svelte';

	import { random } from '$lib/utils';

	const FADE_DURATION = 1000;

	let liked = $state(false);
	let show = $state(false);
	let timeout: ReturnType<typeof setTimeout>;

	function handleClick() {
		liked = !liked;
		if (!liked) return;

		show = true;

		timeout = setTimeout(() => {
			show = false;
		}, FADE_DURATION + 200);
	}

	onMount(() => {
		return () => {
			if (timeout) clearTimeout(timeout);
		};
	});
</script>

<button
	class="group relative rounded-full bg-transparent p-4 hover:bg-gray-50"
	onclick={handleClick}
>
	<svg
		viewBox="0 0 24 24"
		fill="none"
		aria-hidden="true"
		class={[
			'relative size-12 [--heart-color:var(--color-red-400)] group-hover:stroke-(--heart-color)',
			liked ? 'fill-(--heart-color) stroke-(--heart-color)' : 'stroke-gray-400'
		]}
	>
		<path
			d="M3.68546 5.43796C8.61936 1.29159 11.8685 7.4309 12.0406 7.4309C12.2126 7.43091 15.4617 1.29159 20.3956 5.43796C26.8941 10.8991 13.5 21.8215 12.0406 21.8215C10.5811 21.8215 -2.81297 10.8991 3.68546 5.43796Z"
			stroke-width="2"
			stroke-linecap="round"
		/>
	</svg>
	<span class="sr-only">Like this post</span>

	{#if show}
		<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
		{#each { length: 5 } as _, index (index)}
			{@const top = random(0, 100) + '%'}
			{@const left = random(0, 100) + '%'}
			<span
				class="particle absolute size-3 rounded-full bg-red-200"
				style={`top: ${top}; left: ${left}; animation-duration: ${FADE_DURATION + 'ms'};`}
			></span>
		{/each}
	{/if}
</button>

<style>
	@keyframes fadeOut {
		from {
			opacity: 1;
		}
		to {
			opacity: 0;
		}
	}

	.particle {
		animation: fadeOut forwards;
	}
</style>
