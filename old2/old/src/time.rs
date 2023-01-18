// tres &strs formatted as `HH:MM`
pub fn is_between_time(start: &str, end: &str, current: &str) -> bool {
    // if the time is 10:30
    // [10, 30]
    let startvec: Vec<i32> = start
        .split(":")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let endvec: Vec<i32> = end.split(":").map(|x| x.parse::<i32>().unwrap()).collect();
    let currentvec: Vec<i32> = current
        .split(":")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let starttime = construct_time(startvec);
    let endtime = construct_time(endvec);
    let currenttime = construct_time(currentvec);

    // now check if current is between

    // if currenttime is 10:30
    // and starttime is 22:30
    // and endtime is 
    if currenttime.am {
        if !starttime.am
    }
}


struct time {
    time: Vec<i32>,
    am: bool
}
fn construct_time(time: Vec<i32>) -> time {
    time {
        am: {
            // if the hour is less than 24
            // and the hour is more than 12
            // then it is pm
            if time[0] < 24 && time[1] > 12 {
                false
            // else if the hour is greater than 0
            // and less than 12 is it is am
            } else if time[0] > 0 && time[0] < 12 {
                true
            } else {
                panic!("Time ({:?}) is invalid!", time)
            }
        },
        time: time
    }
}