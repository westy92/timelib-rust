mod internal;

use std::{
    ffi::{CStr, CString},
    time::{SystemTime, UNIX_EPOCH},
};

use internal::*;

/// Returns a timestamp (in seconds since the epoch) or an error (string).
///
/// # Arguments
///
/// * `date_time` - A string that holds the relative date you wish to compute.
/// * `base_timestamp` - An optional timestamp (in seconds) to use as your base (defaults to the current timestamp).
/// * `timezone` - An address of a Timezone object.
///
/// # Examples
///
/// ```
/// let tz = timelib::Timezone::parse("America/Chicago".into()).expect("Error parsing timezone!");
/// timelib::strtotime("tomorrow".into(), None, &tz);
/// timelib::strtotime("next tuesday".into(), Some(1654318823), &tz);
/// ```
pub fn strtotime(
    date_time: String,
    base_timestamp: Option<i64>,
    timezone: &Timezone,
) -> Result<i64, String> {
    if date_time.is_empty() {
        return Err("Empty date_time string.".into());
    }

    let date_time_c_str = CString::new(date_time.to_owned());
    if date_time_c_str.is_err() {
        return Err("Malformed date_time string.".into());
    }
    let date_time_c_str = date_time_c_str.unwrap();

    unsafe {
        let mut error = std::mem::MaybeUninit::uninit();
        let parsed_time = timelib_strtotime(
            date_time_c_str.as_ptr(),
            date_time_c_str.to_bytes().len().try_into().unwrap(),
            error.as_mut_ptr(),
            timelib_builtin_db(),
            Some(cached_tzfile_wrapper),
        );
        let err_count = (*error.assume_init()).error_count;
        timelib_error_container_dtor(error.assume_init());
        if err_count != 0 {
            timelib_time_dtor(parsed_time);
            // TODO expose error message(s)
            return Err("Invalid date_time string.".into());
        }

        let base = timelib_time_ctor();
        (*base).tz_info = timezone.tzi;
        (*base).zone_type = TIMELIB_ZONETYPE_ID;
        timelib_unixtime2local(base, base_timestamp.unwrap_or_else(|| rust_now_sec()));

        timelib_fill_holes(parsed_time, base, TIMELIB_NO_CLONE as i32);
        timelib_update_ts(parsed_time, timezone.tzi);
        let result = (*parsed_time).sse;
        timelib_time_dtor(parsed_time);
        timelib_time_dtor(base);

        return Ok(result);
    }
}

unsafe extern "C" fn cached_tzfile_wrapper(
    tz_id: *const i8,
    db: *const timelib_tzdb,
    error: *mut i32,
) -> *mut timelib_tzinfo {
    return timelib_parse_tzfile(tz_id, db, error);
}

fn rust_now_sec() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
}

/// A Timezone wrapper.
#[derive(Debug)]
pub struct Timezone {
    tzi: *mut timelib_tzinfo,
}

impl Drop for Timezone {
    fn drop(&mut self) {
        unsafe {
            timelib_tzinfo_dtor(self.tzi);
        }
    }
}

impl Timezone {
    /// Parses a String into a Timezone instance.
    ///
    /// # Arguments
    ///
    /// * `timezone` - A String with your IANA Timezone name.
    ///
    /// # Examples
    ///
    /// ```
    /// let tz = timelib::Timezone::parse("UTC".into());
    /// let tz = timelib::Timezone::parse("America/Chicago".into());
    /// ```
    pub fn parse(timezone: String) -> Result<Timezone, String> {
        let tz_c_str = CString::new(timezone);
        if tz_c_str.is_err() {
            return Err("Malformed timezone string.".into());
        }
        let tz_c_str = tz_c_str.unwrap();
        let mut error_code: i32 = 0;
        let error_code_ptr = &mut error_code as *mut i32;
        unsafe {
            let tzi = timelib_parse_tzfile(tz_c_str.as_ptr(), timelib_builtin_db(), error_code_ptr);
            if tzi.is_null() {
                return Err(format!("Invalid timezone. Err: {error_code}."));
            }
            Ok(Self { tzi })
        }
    }

    /// Returns the underlying timezone database version.
    pub fn db_version() -> String {
        let cstr = unsafe { CStr::from_ptr((*timelib_builtin_db()).version) };
        String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strtotime_empty_input() {
        let tz = Timezone::parse("UTC".into()).unwrap();
        let result = strtotime("".into(), None, &tz);
        assert!(result.is_err());
        assert_eq!("Empty date_time string.", result.unwrap_err());
    }

    #[test]
    fn strtotime_invalid_date_time() {
        let tz = Timezone::parse("UTC".into()).unwrap();
        let result = strtotime("derp".into(), None, &tz);
        assert!(result.is_err());
        assert_eq!("Invalid date_time string.", result.unwrap_err());
    }

    #[test]
    fn strtotime_invalid_date_time_string() {
        let tz = Timezone::parse("UTC".into()).unwrap();
        let result = strtotime("today\0".into(), None, &tz);
        assert!(result.is_err());
        assert_eq!("Malformed date_time string.", result.unwrap_err());
    }

    #[test]
    fn strtotime_valid_date_time_fixed() {
        let tz = Timezone::parse("UTC".into()).unwrap();
        let result = strtotime("jun 4 2022".into(), None, &tz);
        assert!(result.is_ok());
        assert_eq!(1654300800, result.unwrap());
    }

    #[test]
    fn strtotime_valid_date_time_fixed_timezone() {
        let tz = Timezone::parse("America/Chicago".into()).unwrap();
        let result = strtotime("jun 4 2022".into(), None, &tz);
        assert!(result.is_ok());
        assert_eq!(1654318800, result.unwrap());
    }

    const SEC_PER_DAY: i64 = 86_400;

    #[test]
    fn strtotime_valid_date_time_relative() {
        let tz = Timezone::parse("UTC".into()).unwrap();
        let result = strtotime("tomorrow".into(), None, &tz);
        assert!(result.is_ok());
        let result = result.unwrap();
        let now = rust_now_sec();
        assert!(now <= result);
        assert!(now + SEC_PER_DAY >= result);
    }

    #[test]
    fn strtotime_valid_date_time_relative_base() {
        let tz = Timezone::parse("UTC".into()).unwrap();
        let today = 1654318823; // Saturday, June 4, 2022 5:00:23 AM GMT
        let tomorrow = 1654387200; // Sunday, June 5, 2022 12:00:00 AM GMT
        let result = strtotime("tomorrow".into(), Some(today), &tz);
        assert!(result.is_ok());
        assert_eq!(tomorrow, result.unwrap());
    }

    #[test]
    fn strtotime_valid_date_time_relative_base_timezone() {
        let tz = Timezone::parse("America/Chicago".into()).unwrap();
        let today = 1654318823; // Saturday, June 4, 2022 12:00:23 AM GMT-05:00 DST
        let tomorrow = 1654405200; // Sunday, June 5, 2022 12:00:00 AM GMT-05:00 DST
        let result = strtotime("tomorrow".into(), Some(today), &tz);
        assert!(result.is_ok());
        assert_eq!(tomorrow, result.unwrap());
    }

    #[test]
    fn timezone_invalid_timezone() {
        let result = Timezone::parse("pizza".into());
        assert!(result.is_err());
        assert_eq!("Invalid timezone. Err: 6.", result.unwrap_err());
    }

    #[test]
    fn timezone_invalid_timezone_string() {
        let result = Timezone::parse("UTC\0".into());
        assert!(result.is_err());
        assert_eq!("Malformed timezone string.", result.unwrap_err());
    }

    #[test]
    fn timezone_valid_timezone() {
        let result = Timezone::parse("America/Chicago".into());
        assert!(result.is_ok());
    }

    #[test]
    fn timezone_db_version() {
        assert_eq!("2023.3", Timezone::db_version());
    }
}
