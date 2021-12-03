const DIRECTIONS = {
	FORWARD: 'forward',
	DOWN: 'down',
	UP: 'up',
};

const directionParser = (direction) => {
	const [directionName, directionValue] = direction.split(' ');

	return [directionName, parseInt(directionValue, 10)];
};

export const firstStar = (directions) => {
	let x = 0;
	let depth = 0;

	for (const direction of directions) {
		const [name, value] = directionParser(direction);

		switch (name) {
			case DIRECTIONS.FORWARD:
				x += value;
				break;

			case DIRECTIONS.DOWN:
				depth += value;
				break;

			case DIRECTIONS.UP:
				depth -= value;
				break;
		}
	}

	return depth * x;
};

export const secondStar = (directions) => {
	let x = 0;
	let depth = 0;
	let aim = 0;

	for (const direction of directions) {
		const [name, value] = directionParser(direction);

		switch (name) {
			case DIRECTIONS.FORWARD:
				x += value;
				depth += value * aim;
				break;

			case DIRECTIONS.DOWN:
				aim += value;
				break;

			case DIRECTIONS.UP:
				aim -= value;
				break;
		}
	}

	return depth * x;
};
