<script lang="ts">
	import { MediaQuery } from 'svelte/reactivity';

	import { convertPolarToCartesian, random, range } from '$lib/utils';

	import sparkle from '$lib/assets/particles/exercises/magic-wand/sparkle.svg';

	const DURATION = 1000;
	const FADE_DELAY = 300;
	const WAND_ANGLE = 225;

	let div: HTMLDivElement;
	const enableMotion = new MediaQuery('prefers-reduced-motion: no-preference', false);
	const timers: ReturnType<typeof setTimeout>[] = [];

	function handleClick(e: MouseEvent) {
		const x = e.clientX;
		const y = e.clientY;

		const particles: HTMLImageElement[] = [];
		range(enableMotion.current ? 5 : 3).forEach(() => {
			const particle = document.createElement('img');
			const [polarX, polarY] = enableMotion.current
				? convertPolarToCartesian(random(WAND_ANGLE - 20, WAND_ANGLE + 20), random(30, 90))
				: convertPolarToCartesian(random(WAND_ANGLE - 60, WAND_ANGLE + 60), random(6, 30));

			particle.classList.add('star');
			particle.setAttribute('src', sparkle);
			particle.setAttribute('aria-hidden', 'true');

			particle.style.top = y + 'px';
			particle.style.left = x + 'px';

			particle.style.setProperty('--x', polarX + 'px');
			particle.style.setProperty('--y', polarY + 'px');
			particle.style.setProperty('--duration', (enableMotion.current ? DURATION : 0) + 'ms');
			particle.style.setProperty('--fade-duration', DURATION + 'ms');
			particle.style.setProperty('--fade-delay', FADE_DELAY + 'ms');
			particle.style.setProperty('--rotation', random(90, 360) + 'deg');

			// eslint-disable-next-line svelte/no-dom-manipulating
			div.appendChild(particle);
			particles.push(particle);
		});

		timers.push(
			setTimeout(
				() => {
					particles.forEach((particle) => particle.remove());
				},
				DURATION + FADE_DELAY + 100
			)
		);
	}

	$effect(() => {
		return () => {
			timers.forEach((timer) => clearTimeout(timer));
		};
	});
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div bind:this={div} class="relative h-svh w-svw" onclick={handleClick}></div>

<style>
	@keyframes fadeToTransparent {
		to {
			opacity: 0;
		}
	}

	@keyframes disperse {
		to {
			transform: translate(var(--x), var(--y)) rotate(var(--rotation));
		}
	}

	div {
		cursor:
			url('$lib/assets/particles/exercises/magic-wand/wand.svg') 0 0,
			auto;

		&:active {
			cursor:
				url('$lib/assets/particles/exercises/magic-wand/wand-active.svg') 0 0,
				auto;
		}

		:global(.star) {
			user-select: none;
			pointer-events: none;
			position: absolute;
			animation:
				disperse var(--duration) forwards cubic-bezier(0.26, 0.95, 0, 1),
				fadeToTransparent var(--fade-duration) var(--fade-delay) forwards;
		}
	}
</style>
