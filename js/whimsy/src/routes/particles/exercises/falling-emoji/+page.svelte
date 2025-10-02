<script lang="ts">
	import { random } from '$lib/utils';

	const EMOJIS = ['🎉', '👍', '☕', '🎊', '💸'];
</script>

<main class="overflow-clip rounded-lg pb-8 shadow-lg">
	<div class="relative flex h-30 justify-evenly overflow-hidden bg-pink-200 text-4xl">
		<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
		{#each { length: 8 } as _, index (index)}
			{@const emoji = EMOJIS[Math.round(random(0, EMOJIS.length - 1))]}
			<div
				class="emoji select-none"
				aria-hidden="true"
				style={`--duration: ${Math.round(random(1000, 2000))}ms; --delay: ${Math.round(random(0, 50))}ms;`}
			>
				{emoji}
			</div>
		{/each}
	</div>
	<div
		class="relative isolate mx-auto -mt-16 size-32 overflow-hidden rounded-full border-4 border-white bg-yellow-200"
	>
		<div
			class="absolute top-1 left-7 size-16 translate-y-2 rounded-full border-4 border-white bg-yellow-100"
		></div>
		<div
			class="absolute size-30 translate-y-1/2 rounded-full border-4 border-white bg-yellow-100"
		></div>
	</div>
	<h1 class="m-4 text-center text-xl font-bold">Someone Bought You A Coffee!</h1>
</main>

<style>
	@keyframes fall {
		from {
			transform: translateY(-50%);
		}
		to {
			transform: translateY(150%);
		}
	}

	.emoji {
		animation: fall var(--duration) var(--delay) infinite linear backwards;
	}
</style>
