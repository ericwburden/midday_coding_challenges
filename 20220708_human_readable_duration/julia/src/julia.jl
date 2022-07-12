module julia

export format_duration

const MINUTE = 60
const HOUR   = 60  * MINUTE
const DAY    = 24  * HOUR
const YEAR   = 365 * DAY

const UNIT_DURATIONS = [
    (YEAR,   "year"),
    (DAY,    "day"),
    (HOUR,   "hour"),
    (MINUTE, "minute"),
    (1,      "second")
]

const Duration = Tuple{Int,String}

function to_label(duration::Duration)::String
    (n, label) = duration
    n == 0 && return missing
    n == 1 && return "1 $label"
    "$n $(label)s"
end

function format_duration(seconds::Int)::String
    seconds == 0 && return "now"
    parts = []

    for (duration_seconds, name) in UNIT_DURATIONS
        (count, seconds) = divrem(seconds, duration_seconds)
        count > 0 && push!(parts, to_label((count, name)))
    end

    join(parts, ", ", " and ")
end

end #julia