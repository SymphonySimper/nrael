<script lang="ts">
	import { lerp } from '$lib/utils';
	import type { FormEventHandler } from 'svelte/elements';

	const RANGE_MIN = 0;
	const RANGE_MAX = 100;

	let box: HTMLDivElement | undefined = $state();

	function getLerp(value: number, min: number, max: number): number {
		return lerp({
			value: value,
			currentScaleMin: RANGE_MIN,
			currentScaleMax: RANGE_MAX,
			newScaleMin: min,
			newScaleMax: max
		});
	}

	function transformBox(value: number) {
		if (!box) return;

		// “skew” should range from 25° to 0°
		const skew = getLerp(value, 25, 0);
		// “rotate” should range from 225° to -45°
		const rotate = getLerp(value, 225, -45);
		// “radius” should range from 50% to 5%
		const radius = getLerp(value, 50, 5);
		// “scaleY” should grow from 0.01 to 1 over the
		//  *first half* of the range. When “value” is
		//  50+, scale should stay at 1.
		const scaleY = value >= 50 ? 1 : getLerp(value, 0.01, 1);
		// “boxHue” should stay at 0° for the first half
		// of the range, and then scale from 0° to 45°
		// over the *second half* of the range.
		const boxHue = value <= 50 ? 0 : getLerp(value, 0, 45);
		// “bgLightness” should stay at 6% for the first 75%
		//  of the range, and then scale from 6% to 26%
		//  over the final 25% of the range.
		const bgLightness = value <= 75 ? 6 : getLerp(value, 6, 26);

		// Alternate method using clamp
		// const scaleY = clampedLerp({
		// 	value,
		// 	currentScaleMin: 0,
		// 	currentScaleMax: 50,
		// 	newScaleMin: 0.01,
		// 	newScaleMax: 1
		// });
		// const boxHue = clampedLerp({
		// 	value,
		// 	currentScaleMin: 50,
		// 	currentScaleMax: 100,
		// 	newScaleMin: 0,
		// 	newScaleMax: 45
		// });
		// const bgLightness = clampedLerp({
		// 	value,
		// 	currentScaleMin: 75,
		// 	currentScaleMax: 100,
		// 	newScaleMin: 6,
		// 	newScaleMax: 26
		// });

		// No changes necessary below this point.
		box.style.transform = `
    scaleY(${scaleY})
    rotate(${rotate}deg)
    skewX(${skew}deg)
  `;
		box.style.borderRadius = radius + '%';
		box.style.backgroundColor = `hsl(${boxHue}deg 100% 60%)`;

		document.documentElement.style.backgroundColor = `hsl(210deg 15% ${bgLightness}%)`;
	}

	const handleInput: FormEventHandler<HTMLInputElement> = (e): void => {
		transformBox(Number(e.currentTarget.value));
	};

	$effect(() => {
		if (!box) return;

		transformBox(0);
	});
</script>

<div bind:this={box} class="size-40"></div>

<form class="fixed right-0 bottom-8 left-0 mx-auto w-fit">
	<label class="flex flex-col gap-1 text-center">
		<span class="font-medium">Reveal:</span>
		<input
			type="range"
			oninput={handleInput}
			min={RANGE_MIN}
			max={RANGE_MAX}
			class="w-screen max-w-80"
		/>
	</label>
</form>
