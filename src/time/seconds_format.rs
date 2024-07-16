pub enum SecondsFormat {
    /// Format whole seconds only, with no decimal point nor subseconds.
    Secs,

    /// Use fixed 3 subsecond digits.
    Millis,

    /// Use fixed 6 subsecond digits.
    Micros,

    /// Use fixed 9 subsecond digits.
    Nanos,
}
