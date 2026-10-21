/// This is a comment.
/// Continued lovingly here
#[typeshare]
pub enum Colors {
    Red = 0,
    Blue = 1,
    /// Green is a cool color
    Green = 2,
}

#[typeshare]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogLevel {
    Error,
    #[serde(rename = "warn\"ing")]
    Warning,
    DebugTrace,
}

#[typeshare]
pub enum Uninhabited {}

/// Algebraic enums are not affected by the option.
#[typeshare]
#[serde(tag = "type", content = "content")]
pub enum Shape {
    Circle(f64),
    Point,
}
