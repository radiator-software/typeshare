#[typeshare]
pub struct Square {
    side: f64,
}

/// Shapes tagged inside their own object
#[typeshare]
#[serde(tag = "type")]
pub enum Shape {
    /// A shape without data
    Empty,
    /// A struct variant
    Circle {
        /// Radius of the circle
        radius: f64,
        label: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Rectangle {
        side_width: f64,
        side_height: f64,
    },
    /// A newtype variant
    Square(Square),
    Point(()),
    Labels(HashMap<String, String>),
    #[serde(skip)]
    Skipped(String),
}

#[typeshare]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Event<T> {
    Started,
    Payload(T),
    Wrapped { inner: T },
}

/// Unit-only enums with a tag serialize as objects
#[typeshare]
#[serde(tag = "message-type")]
pub enum Ping {
    Ping,
    Pong,
}
