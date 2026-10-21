/**
 * This is a comment.
 * Continued lovingly here
 */
export type Colors =
	| "Red"
	| "Blue"
	/** Green is a cool color */
	| "Green";

export type LogLevel =
	| "ERROR"
	| "warn\"ing"
	| "DEBUG_TRACE";

/** Algebraic enums are not affected by the option. */
export type Shape = 
	| { type: "Circle", content: number }
	| { type: "Point", content?: undefined };

export type Uninhabited = never;

