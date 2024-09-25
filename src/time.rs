use chrono::prelude::*;

pub fn get_last_lesson() -> NaiveTime {
    let now = Local::now().naive_local().time();
    let last_lesson = lesson_starts().into_iter().filter(|&time| time < now).max();

    match last_lesson {
        Some(time) => time,
        None => lesson_starts()[lesson_starts().len() - 1],
    }
}
pub fn get_last_lesson_t(t: NaiveTime) -> NaiveTime {
    let now = t;
    let last_lesson = lesson_starts()
        .into_iter()
        .filter(|&time| time <= now)
        .max();

    match last_lesson {
        Some(time) => time,
        None => lesson_starts()[lesson_starts().len() - 1],
    }
}

pub fn get_prev_lesson(t: NaiveTime) -> NaiveTime {
    let lessons = lesson_starts();
    let last_lesson = get_last_lesson_t(t);

    // Find the index of the last lesson
    let mut index = lessons.len(); // Set to len() as a fallback
    for (i, lesson) in lessons.iter().enumerate() {
        if *lesson == last_lesson {
            index = i;
            break;
        }
    }

    // Return the previous lesson, cycling back to the last one if necessary
    if index == 0 {
        lessons[lessons.len() - 1]
    } else {
        lessons[index - 1]
    }
}

pub fn get_next_lesson(t: NaiveTime) -> NaiveTime {
    let lessons = lesson_starts();
    let last_lesson = get_last_lesson_t(t);

    // Find the index of the last lesson
    let mut index = lessons.len(); // Set to len() as a fallback
    for (i, lesson) in lessons.iter().enumerate() {
        if *lesson == last_lesson {
            index = i;
            break;
        }
    }

    // Return the next lesson, cycling back to the first one if necessary
    if index == lessons.len() - 1 {
        lessons[0]
    } else {
        lessons[index + 1]
    }
}

pub fn get_today() -> NaiveDate {
    Local::now().date_naive()
}

fn lesson_starts() -> Vec<NaiveTime> {
    let times = [
        (7, 45),
        (8, 40),
        (9, 35),
        (10, 35),
        (11, 30),
        (12, 25),
        (13, 20),
        (14, 15),
        (15, 10),
        (16, 05),
    ];
    times
        .into_iter()
        .map(|x| NaiveTime::from_hms_opt(x.0, x.1, 0).expect("Couldnt create time"))
        .collect()
}
