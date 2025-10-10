<div class="relative flex gap-4 rounded-[5px] border-b-5 border-gray-400 px-4">
	<!-- Ball 1 should bounce 30px over 300ms -->
	<div class="ball" style="--distance: 30px; --duration: 300ms;"></div>

	<!-- Ball 2 should bounce 50px over 400ms -->
	<div class="ball" style="--distance: 50px; --duration: 400ms;"></div>

	<!-- Ball 3 should bounce 70px over 500ms -->
	<div class="ball" style="--distance: 70px; --duration: 500ms;"></div>
</div>

<style>
	@keyframes bounce {
		to {
			transform: translateY(calc(var(--distance) * -1));
		}
	}

	.ball {
		width: 50px;
		height: 50px;
		border-radius: 45%;
		background: hsl(45deg 100% 50%);
		background: oklch(0.87 0.2 90);
		animation: bounce var(--duration) infinite alternate cubic-bezier(0.22, 0.25, 0.08, 0.96);
	}
</style>
