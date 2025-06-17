
// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
// pub struct LocalTimeTypeRecord {
//     /// A signed integer specifying the number of
//     /// seconds to be added to UT in order to determine local time.
//     pub utoff: Seconds,

//     /// A value indicating whether local time should
//     /// be considered Daylight Saving Time (DST).  The value MUST be 0
//     /// A value of [`true`] indicates that this type of time is DST.
//     /// A value of [`false`] indicates that this time type is standard time.
//     pub is_dst: bool,

//     /// An unsigned integer specifying a zero-based
//     /// index into the series of time zone designation bytes, thereby
//     /// selecting a particular designation string.  Each index MUST be
//     /// in the range [0, "charcnt" - 1]; it designates the
//     /// NUL-terminated string of bytes starting at position "idx" in
//     /// the time zone designations.  (This string MAY be empty.)  A NUL
//     /// byte MUST exist in the time zone designations at or after
//     /// position "idx".
//     pub idx: usize,
// }

// #[derive(Debug, Clone, Default)]
// pub struct TzTransitionInfo {
//     pub transition_times: Vec<i64>,

//     pub transition_types: Vec<usize>,

//     pub local_time_type_records: Vec<LocalTimeTypeRecord>,
// }