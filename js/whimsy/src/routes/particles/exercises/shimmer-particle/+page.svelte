<script lang="ts">
	import { onMount } from 'svelte';

	const SHIMMER_DURATION = 1000;

	const timeouts: ReturnType<typeof setTimeout>[] = [];
	let button: HTMLButtonElement;

	function handleShowShimmer() {
		if (!button) return;

		const shimmer = document.createElement('span');
		shimmer.style.animationDuration = SHIMMER_DURATION + 'ms';

		// eslint-disable-next-line svelte/no-dom-manipulating
		button.appendChild(shimmer);

		timeouts.push(
			setTimeout(() => {
				shimmer.remove();
			}, SHIMMER_DURATION + 200)
		);
	}

	onMount(() => {
		return () => {
			timeouts.forEach((timeout) => clearTimeout(timeout));
		};
	});
</script>

<button
	bind:this={button}
	onmouseenter={handleShowShimmer}
	onfocus={handleShowShimmer}
	class="relative isolate block cursor-pointer overflow-hidden rounded-full px-12 py-4 font-medium text-white"
>
	Buy Now
</button>

<style>
	@keyframes shimmer {
		from {
			transform: translateX(-100%);
		}

		to {
			transform: translateX(100%);
		}
	}

	button {
		background:
			radial-gradient(
				circle at 50% 0%,
				oklch(0.5 0.15 165 / 0) 0% 60%,
				oklch(0.5 0.15 165 / 0.125) 70%,
				oklch(0.5 0.15 165 / 0.325) 80%,
				oklch(0.5 0.15 165 / 0) 100%
			),
			linear-gradient(to top, oklch(0.6 0.19 164 / 1), oklch(0.9 0.2 182 / 1));
		box-shadow:
			inset 0px -1px 2px hsl(175deg 100% 20% / 0.5),
			inset 0px -3px 6px hsl(175deg 100% 20% / 0.4),
			inset 0px 5px 5px hsl(175deg 100% 90% / 0.75);

		/*
    Add a white glow near the top of the button,
    to make the button feel a bit more 3D and gel-like:
  */
		&::before {
			content: '';
			position: absolute;
			top: 4px;
			left: 16px;
			right: 16px;
			height: 10px;
			border-radius: 100px 100px 0 0;
			border-top: 4px solid white;
			filter: blur(4px);
			opacity: 0.625;
		}

		:global(span) {
			position: absolute;
			width: 100%;
			height: 100%;
			inset: 0;
			background: linear-gradient(to right, transparent, hsl(180deg 100% 90%), transparent);
			animation: shimmer both;
			opacity: 0.5;
		}
	}
</style>
