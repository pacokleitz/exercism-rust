use time::Duration;
use time::PrimitiveDateTime as DateTime;

const GIGA_SECOND: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGA_SECOND)
}
