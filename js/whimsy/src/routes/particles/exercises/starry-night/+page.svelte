<script lang="ts">
	import { random, range } from '$lib/utils';

	const STAR = '⭐';
	const STAR_COUNT = 10;

	let starBoard: HTMLDivElement;

	function handleAddStars() {
		if (!starBoard) return;

		range(STAR_COUNT).forEach(() => {
			const star = document.createElement('div');
			star.classList.add('star');

			star.style.top = random(0, 100) + '%';
			star.style.left = random(0, 100) + '%';

			star.textContent = STAR;

			// eslint-disable-next-line svelte/no-dom-manipulating
			starBoard.appendChild(star);
		});
	}
</script>

<div bind:this={starBoard} class="relative isolate h-svh w-svw overflow-hidden bg-gray-900"></div>

<button
	class="fixed right-0 bottom-4 left-0 mx-auto w-fit rounded-full border-r-4 border-b-4 border-blue-700 bg-blue-500 px-8 py-4 font-medium text-white uppercase active:border-r active:border-b"
	onclick={handleAddStars}
>
	add stars {STAR}
</button>

<style>
	@keyframes fadeIn {
		from {
			opacity: 0;
		}

		to {
			opacity: 1;
		}
	}

	div :global(.star) {
		position: fixed;
		display: flex;
		animation: fadeIn 1000ms;
		pointer-events: none;
	}
</style>
