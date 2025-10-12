<script lang="ts">
	import { innerWidth, innerHeight } from 'svelte/reactivity/window';

	import { lerp } from '$lib/utils';

	let horizontal: string = $state('--');
	let vertical: string = $state('--');

	function handleMove(e: PointerEvent) {
		if (innerWidth.current === undefined || innerHeight.current === undefined) return;

		const x = lerp({
			value: e.clientX,
			currentScaleMin: 0,
			currentScaleMax: innerWidth.current,
			newScaleMin: 0,
			newScaleMax: 100
		});
		const y = lerp({
			value: e.clientY,
			currentScaleMin: 0,
			currentScaleMax: innerHeight.current,
			newScaleMin: 0,
			newScaleMax: 100
		});

		horizontal = Math.round(x) + '%';
		vertical = Math.round(y) + '%';
	}
</script>

<svelte:window onpointermove={handleMove} />

{#snippet li(label: string, value: string)}
	<li class="flex flex-1 flex-col items-center">
		<span class="text-base text-gray-500">{label}</span>
		<span class="text-lg font-bold">{value}</span>
	</li>
{/snippet}

<h1 class="mb-4 text-center text-2xl font-bold">Mouse Position</h1>
<ul class="flex justify-center gap-8">
	{@render li('Horizontal', horizontal)}
	{@render li('Vertical', vertical)}
</ul>
