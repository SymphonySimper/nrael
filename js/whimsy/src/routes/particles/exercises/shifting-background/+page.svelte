<script lang="ts">
	import { innerHeight, innerWidth } from 'svelte/reactivity/window';

	import { lerp } from '$lib/utils';

	let div: HTMLDivElement;

	function handleMove(e: PointerEvent): void {
		if (innerWidth.current === undefined || innerHeight.current === undefined) return;

		// hue range between 50deg and 200deg, from
		// left to right, based on “x” position:
		const hue = lerp({
			value: e.clientX,
			currentScaleMin: 0,
			currentScaleMax: innerWidth.current,
			newScaleMin: 50,
			newScaleMax: 200
		});

		// saturation range between 100% and 25%,
		// from top to bottom, based on “y” position:
		const saturation = lerp({
			value: e.clientY,
			currentScaleMin: 0,
			currentScaleMax: innerHeight.current,
			newScaleMin: 100,
			newScaleMax: 25
		});

		div.style.background = `hsl(${hue}deg ${saturation}% 75%)`;
	}
</script>

<div bind:this={div} class="h-svh w-svw content-center text-center" onpointermove={handleMove}>
	<h1 class="text-2xl font-bold">Shifting Background</h1>
</div>
