<script lang="ts">
	import { resolve } from '$app/paths';

	const LINKS = ['particles'] as const;
</script>

<div class="flex flex-col gap-0 md:w-[50vw]">
	<h1 class="border-b border-gray-200 p-2 text-2xl text-gray-800">Links:</h1>
	<div class="grid grid-cols-2 p-2">
		{#each LINKS as link, index (link)}
			{@const resolvedLink = resolve(`/${link}`)}
			<a
				href={resolvedLink}
				class="text-lg font-bold capitalize hover:text-blue-500 hover:underline"
			>
				<span>{index + 1}.</span>
				{link}</a
			>
		{/each}
	</div>
</div>
