export interface Square {
	side: number;
}

export type Event<T> = 
	| { kind: "started" }
	| ({ kind: "payload" } & T)
	| {
	kind: "wrapped";
	inner: T;
};

/** Unit-only enums with a tag serialize as objects */
export type Ping = 
	| { "message-type": "Ping" }
	| { "message-type": "Pong" };

/** Shapes tagged inside their own object */
export type Shape = 
	/** A shape without data */
	| { type: "Empty" }
	/** A struct variant */
	| {
	type: "Circle";
	/** Radius of the circle */
	radius: number;
	label?: string;
}
	| {
	type: "Rectangle";
	sideWidth: number;
	sideHeight: number;
}
	/** A newtype variant */
	| ({ type: "Square" } & Square)
	| { type: "Point" }
	| ({ type: "Labels" } & Record<string, string>);

