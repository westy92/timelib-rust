#include "../ext/timelib/timelib.h"

/**
 * A function that creates timelib_tzinfos if they don't exist and caches them in memory.
*/
timelib_tzinfo *timelib_tz_get_wrapper_cached(const char *tzname, const timelib_tzdb *tzdb, int *error_code);
