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

export function sample<D>(values: Array<D>): D {
	return values[Math.round(random(0, values.length - 1))];
}

// Liner Interpolation
type LerpProps = {
	value: number;
	currentScaleMin: number;
	currentScaleMax: number;
	newScaleMin?: number;
	newScaleMax?: number;
};

function getLerpProps(props: LerpProps): Required<LerpProps> {
	return { ...props, newScaleMin: props.newScaleMin ?? 0, newScaleMax: props.newScaleMax ?? 1 };
}

export function lerp(props: LerpProps): number {
	const { value, currentScaleMin, currentScaleMax, newScaleMin, newScaleMax } = getLerpProps(props);
	const standardNormalization = (value - currentScaleMin) / (currentScaleMax - currentScaleMin);

	return (newScaleMax - newScaleMin) * standardNormalization + newScaleMin;
}

export function clamp(value: number, min: number = 0, max: number = 1): number {
	if (min > max) {
		[min, max] = [max, min];
	}

	return Math.max(min, Math.min(max, value));
}

export function clampedLerp(props: LerpProps) {
	const { value, currentScaleMin, currentScaleMax, newScaleMin, newScaleMax } = getLerpProps(props);
	return clamp(
		lerp({
			value,
			currentScaleMin,
			currentScaleMax,
			newScaleMin,
			newScaleMax
		}),
		newScaleMin,
		newScaleMax
	);
}
