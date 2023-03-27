interface parseOptions {
	sep?: string;
}

export const parse = (data: string, options: parseOptions = {}) => {
	if (options.sep) {
		let lines = data.split(/\r\n|\n|\r/);
		return lines.map((value) => value.split(options.sep as string));
	}
	return data.split(/\r\n|\n|\r/).map((el) => [el]);
};
