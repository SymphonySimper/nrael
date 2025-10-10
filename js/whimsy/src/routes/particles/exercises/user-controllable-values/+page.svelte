<script lang="ts">
	import type { FormEventHandler } from 'svelte/elements';

	let bar: HTMLDivElement;

	const handleInput: FormEventHandler<HTMLInputElement> = (e): void => {
		const angle = e.currentTarget.value;
		bar.style.setProperty('--angle', angle + 'deg');
	};
</script>

<div bind:this={bar} class="h-10 w-40 rounded-sm bg-yellow-400"></div>

<form class="fixed right-0 bottom-8 left-0 mx-auto w-fit">
	<label class="flex flex-col gap-1 text-center">
		<span class="font-medium">ROTATE AMOUNT</span>
		<input type="range" oninput={handleInput} min="0" max="180" class="w-screen max-w-80" />
	</label>
</form>

<style>
	@keyframes back-and-forth {
		from {
			transform: rotate(var(--angle));
		}
		to {
			transform: rotate(calc(var(--angle) * -1));
		}
	}

	div {
		--angle: 0deg;
		animation: back-and-forth 1000ms infinite alternate ease-in-out;
	}
</style>
