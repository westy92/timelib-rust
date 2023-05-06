mod internal;

use std::time::{SystemTime, UNIX_EPOCH};

use internal::*;

pub fn strtotime(
    date_time: String,
    base_timestamp: Option<i64>,
    timezone: Option<String>,
) -> Result<i64, String> {
    if date_time.is_empty() {
        return Err("Empty input string.".into());
    }

    let tz = timezone.unwrap_or("UTC".into());
    let mut error_code: i32 = 0;
    let error_code_ptr = &mut error_code as *mut i32;

    unsafe {
        let tzi = timelib_parse_tzfile(
            tz.as_ptr() as *const i8,
            timelib_builtin_db(),
            error_code_ptr,
        );
        if tzi.is_null() {
            return Err("Invalid timezone.".into());
        }

        let base = timelib_time_ctor();
        (*base).tz_info = tzi;
        (*base).zone_type = TIMELIB_ZONETYPE_ID;
        timelib_unixtime2local(base, base_timestamp.unwrap_or_else(|| rust_now_sec()));

        let mut error = std::mem::MaybeUninit::uninit();
        let parsed_time = timelib_strtotime(
            date_time.as_ptr() as *const i8,
            date_time.len().try_into().unwrap(),
            error.as_mut_ptr(),
            timelib_builtin_db(),
            Some(cached_tzfile_wrapper),
        );
        let err_count = (*error.assume_init()).error_count;
        timelib_error_container_dtor(error.assume_init());
        if err_count != 0 {
            timelib_time_dtor(parsed_time);
            timelib_tzinfo_dtor(tzi);
            return Err("Invalid date_time string.".into());
        }
        timelib_fill_holes(parsed_time, base, TIMELIB_NO_CLONE as i32);
        timelib_update_ts(parsed_time, tzi);
        let result = timelib_date_to_int(parsed_time, error_code_ptr);
        timelib_time_dtor(parsed_time);
        timelib_time_dtor(base);
        timelib_tzinfo_dtor(tzi);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtotime_empty_input() {
        let result = strtotime("".into(), None, None);
        assert!(result.is_err());
        assert_eq!("Empty input string.", result.unwrap_err());
    }

    #[test]
    fn test_strtotime_invalid_timezone() {
        let result = strtotime("today".into(), None, Some("pizza".into()));
        assert!(result.is_err());
        assert_eq!("Invalid timezone.", result.unwrap_err());
    }

    /*#[test]
    fn test_strtotime_invalid_date_time() {
        let result = strtotime("derp".into(), None, None);
        assert!(result.is_err());
        assert_eq!("Invalid date_time string.", result.unwrap_err());
    }*/

    // TODO more tests
}
