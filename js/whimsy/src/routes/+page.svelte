<script lang="ts">
	import { resolve } from '$app/paths';

	const LINKS = [{ topic: 'particles', exercises: ['starry-night'] }] as const;
</script>

<div class="flex flex-col gap-2 divide-y divide-gray-200 p-2 md:w-[50vw]">
	<h1 class="text-2xl text-gray-800">Links</h1>
	{#each LINKS as link, index (link)}
		{@const resolvedTopicLink = resolve(`/${link.topic}`)}
		<div class="flex flex-col gap-1">
			<a
				href={resolvedTopicLink}
				class="text-lg font-bold capitalize hover:text-blue-500 hover:underline"
			>
				<span>{index + 1}.</span>
				{link.topic}</a
			>
			{#if link.exercises.length > 0}
				<div class="flex items-center gap-1">
					<div class="text-gray-500">Exercises:</div>
					{#each link.exercises as exercise (exercise)}
						{@const resolvedLink = resolve(`/${link.topic}/exercises/${exercise}`)}
						<a
							href={resolvedLink}
							class="rounded-lg border border-gray-200 bg-gray-50 p-1 text-sm hover:underline"
							>{exercise}</a
						>
					{/each}
				</div>
			{/if}
		</div>
	{/each}
</div>
