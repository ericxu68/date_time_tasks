use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, Utc};
use std::thread;
use std::time::{Duration as duree, Instant};

fn expensive_function() {
    thread::sleep(duree::from_secs(1));
}

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn main() {
    /******************
     * TIME ONE'S CODE
     ******************/
    // time ones code
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    /*******************************
     * GET DATE THREE WEEKS FROM NOW
     *******************************/

    let now = Utc::now();
    println!("{}", now);

    let three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|two_weeks| two_weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);
    // use match to unpack the Option
    match three_weeks_from_now {
        Some(three_weeks_from_now) => {
            println!("{}", three_weeks_from_now);
        }
        None => {
            eprint!("Almost three weeks from now overflows!");
        }
    }
    // println!("{}", three_weeks_from_now.unwrap());

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }

    /*************************************
     * CONVERT LOCAL TIMEZONE TO ANOTHER
     *************************************/

    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let timezone_east = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    let naivedatetime_east = NaiveDate::from_ymd_opt(2000, 1, 12)
        .unwrap()
        .and_hms_opt(10, 0, 0)
        .unwrap();
    let datetime_east = DateTime::<FixedOffset>::from_local(naivedatetime_east, timezone_east);
    println!("Eastern time is: {}", datetime_east);

    let timezone_west = FixedOffset::west_opt(7 * 60 * 60).unwrap();
    let naivedatetime_west = NaiveDate::from_ymd_opt(2000, 1, 11)
        .unwrap()
        .and_hms_opt(19, 0, 0)
        .unwrap();
    let datetime_west = DateTime::<FixedOffset>::from_local(naivedatetime_west, timezone_west);
    println!("Western time is: {}", datetime_west);
}
