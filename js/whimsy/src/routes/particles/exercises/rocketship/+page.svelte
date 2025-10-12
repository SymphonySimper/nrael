<script lang="ts">
	import { convertPolarToCartesian, random, sample } from '$lib/utils';

	import clayRocket from '$lib/assets/particles/exercises/rocketship/clay-rocket.png';

	const COLORS = [
		'hsl(35deg 100% 50%)',
		'hsl(40deg 100% 50%)',
		'hsl(45deg 100% 60%)',
		'hsl(50deg 100% 65%)'
	];
	const DURATION = 500;
	const DELAY = 500;

	let div: HTMLDivElement | undefined = $state();
	const timers: ReturnType<typeof setTimeout>[] = [];
	let interval: ReturnType<typeof setInterval> | null = null;

	$effect(() => {
		if (!div) return;

		interval = setInterval(() => {
			const particle = document.createElement('div');
			particle.classList.add('particle');
			particle.style.backgroundColor = sample(COLORS);

			const [polarX, polarY] = convertPolarToCartesian(random(90 - 30, 90 + 30), random(45, 80));
			particle.style.setProperty('--x', polarX + 'px');
			particle.style.setProperty('--y', polarY + 'px');
			particle.style.setProperty('--disperse-duration', DURATION + 250 + 'ms');
			particle.style.setProperty('--fade-delay', DELAY + 'ms');
			particle.style.setProperty('--fade-duration', DURATION + 'ms');

			// eslint-disable-next-line svelte/no-dom-manipulating
			div?.prepend(particle);

			timers.push(setTimeout(() => particle.remove(), DURATION + DELAY + 200));
		}, 50);

		return () => {
			if (interval !== null) clearInterval(interval);

			timers.forEach((timer) => clearTimeout(timer));
		};
	});
</script>

<div class="grid h-svh w-svw place-content-center bg-[hsl(220deg_50%_8%)]">
	<div class="wrapper relative" bind:this={div}>
		<img alt="Clay Rocket" src={clayRocket} />
	</div>
</div>

<style>
	@keyframes oscillate {
		from {
			transform: translateY(-5%);
		}
		to {
			transform: translateY(5%);
		}
	}

	@keyframes fadeToTransparent {
		to {
			opacity: 0;
		}
	}

	@keyframes disperse {
		to {
			transform: translate(var(--x), var(--y));
		}
	}

	.wrapper {
		img {
			width: 125px;
			max-width: 100%;
			aspect-ratio: 500 / 503;
			animation: oscillate 1500ms infinite alternate ease-in-out;
		}

		:global(.particle) {
			position: absolute;
			width: 10px;
			height: 10px;
			border-radius: 50%;
			left: 0;
			right: 0;
			margin-inline: auto;
			bottom: 20px;
			user-select: none;
			pointer-events: none;
			animation:
				disperse var(--disperse-duration) forwards ease-out,
				fadeToTransparent var(--fade-duration) var(--fade-delay) forwards;
		}
	}
</style>
