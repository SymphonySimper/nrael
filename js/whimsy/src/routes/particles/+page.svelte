<script lang="ts">
	import { onMount } from 'svelte';

	import { random, range } from '$lib/utils';

	const FADE_DURATION = 1000;

	let button: HTMLButtonElement;
	let liked = $state(false);
	let timeouts: ReturnType<typeof setTimeout>[] = [];

	function handleClick() {
		if (!button) return;

		liked = !liked;
		if (!liked) return;

		const particles: HTMLSpanElement[] = [];

		range(5).forEach(() => {
			const particle = document.createElement('span');

			particle.classList.add('particle');
			particle.style.top = random(0, 100) + '%';
			particle.style.left = random(0, 100) + '%';
			particle.style.animationDuration = FADE_DURATION + 'ms';

			// eslint-disable-next-line svelte/no-dom-manipulating
			button.appendChild(particle);
			particles.push(particle);
		});

		timeouts.push(
			setTimeout(() => {
				particles.forEach((particle) => particle.remove());
			}, FADE_DURATION + 200)
		);
	}

	onMount(() => {
		return () => {
			timeouts.forEach((timeout) => {
				clearInterval(timeout);
			});
		};
	});
</script>

<button
	bind:this={button}
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
</button>

<style>
	@keyframes fadeToTransparent {
		to {
			opacity: 0;
		}
	}

	button :global(.particle) {
		--size: calc(var(--spacing) * 3);
		position: absolute;
		height: var(--size);
		width: var(--size);
		transform: translate(-50%, -50%);
		border-radius: 50%;
		background: var(--color-red-200);
		animation: fadeToTransparent forwards;
	}
</style>
