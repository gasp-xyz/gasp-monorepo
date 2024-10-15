import { createLogger, format, transports } from "winston";
import { LOG } from "./config.js";

const myFormat = format.printf(({ level, message, label, timestamp }) => {
	return `${timestamp} ${level}: ${message}`;
});

const logger = createLogger({
	level: LOG,
	format: format.combine(format.timestamp(), format.colorize(), myFormat),
	transports: [new transports.Console()],
});

export { logger };
