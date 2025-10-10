export function range(start: number, end?: number, step = 1): number[] {
	const output = [];

	if (end === undefined) {
		end = start;
		start = 0;
	}

	for (let i = start; i < end; i += step) {
		output.push(i);
	}

	return output;
}

export function random(min: number, max: number): number {
	return Math.random() * (max - min) + min;
}

export function convertDegreesToRadians(angle: number): number {
	return (angle * Math.PI) / 180;
}

export function convertPolarToCartesian(angle: number, distance: number): [number, number] {
	const radians = convertDegreesToRadians(angle);

	const x = Math.cos(radians) * distance;
	const y = Math.sin(radians) * distance;

	return [x, y];
}
