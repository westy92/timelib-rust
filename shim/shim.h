#include "../ext/hashmap.h/hashmap.h"
#include "../ext/timelib/timelib.h"

struct hashmap_s hashmap;
bool hm_init = false;

timelib_tzinfo *timelib_tz_get_wrapper_cached(const char *tzname, const timelib_tzdb *tzdb, int *error_code) {
    if (!hm_init)
    {
        if (0 != hashmap_create(1, &hashmap)) {
        // error!
        // TODO what to do?
        return 0;
        }
        else
        {
        hm_init = true;
        }
    }
    int tzname_len = strlen(tzname);
    void *const existing = hashmap_get(&hashmap, tzname, tzname_len);
    if (NULL != existing) {
        return (timelib_tzinfo*) existing;
    }
    timelib_tzinfo *tzi = timelib_parse_tzfile(tzname, tzdb, error_code);
    if (tzi) {
        hashmap_put(&hashmap, tzname, tzname_len, tzi);
    }

    return tzi;
}
