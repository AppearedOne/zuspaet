use crate::App;
use crate::{
    stats::Ranking,
    time::{self, get_today},
};
use chrono::{NaiveDate, NaiveTime};
use serde_derive::*;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct DataBaseError;

impl From<io::Error> for DataBaseError {
    fn from(_value: io::Error) -> Self {
        DataBaseError {}
    }
}
impl From<serde_json::Error> for DataBaseError {
    fn from(_value: serde_json::Error) -> Self {
        DataBaseError {}
    }
}

pub struct ProfileStats {
    pub person: Class,
    pub num: u32,
    pub avg_min: f32,
    pub percent: f32,
    pub min: u32,
    pub max: u32,
    pub latest: Option<Entry>,
    pub theo_penalties: u32,
    pub sum: u32,
    pub first_lesson_percent: f32,
}

impl ProfileStats {
    pub fn empty(p: Class) -> Self {
        ProfileStats {
            person: p,
            num: 0,
            avg_min: 0.0,
            percent: 0.0,
            min: 0,
            max: 0,
            latest: None,
            theo_penalties: 0,
            sum: 0,
            first_lesson_percent: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Entry {
    pub person: Class,
    pub lesson: Lesson,
    pub lesson_time: NaiveTime,
    pub delay_min: u32,
    pub first_lesson: bool,
    pub date: NaiveDate,
}
impl Entry {
    pub fn empty() -> Entry {
        Entry {
            person: Class::Marie,
            lesson: Lesson::BG,
            lesson_time: time::get_last_lesson(),
            delay_min: 0,
            first_lesson: false,
            date: get_today(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonAbs {
    pub present: Vec<Class>,
    pub lesson: Lesson,
    pub lesson_time: NaiveTime,
    pub first_lesson: bool,
    pub date: NaiveDate,
}

impl Default for LessonAbs {
    fn default() -> Self {
        LessonAbs::new()
    }
}
impl LessonAbs {
    pub fn new() -> Self {
        LessonAbs {
            present: Class::all(),
            lesson: Lesson::BG,
            lesson_time: time::get_last_lesson(),
            first_lesson: false,
            date: get_today(),
        }
    }
    pub fn new_smart(absences: &Vec<LessonAbs>) -> Self {
        let mut l = LessonAbs {
            present: Class::all(),
            lesson: Lesson::BG,
            lesson_time: time::get_last_lesson(),
            first_lesson: false,
            date: get_today(),
        };
        if absences.len() > 0 {
            l.present = absences[absences.len() - 1].present.clone();
            l.lesson = absences[absences.len() - 1].lesson.clone();
        }
        l
    }
    pub fn toggle_person(&mut self, person: Class) {
        if self.present.contains(&person) {
            self.present.retain(|p| *p != person);
        } else {
            self.present.push(person);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataBase {
    pub data: Vec<Entry>,
    pub absences: Vec<LessonAbs>,
}
impl DataBase {
    pub fn empty() -> DataBase {
        DataBase {
            data: vec![],
            absences: vec![],
        }
    }
    pub async fn load_file(path: &str) -> Result<DataBase, DataBaseError> {
        let content = std::fs::read_to_string(path)?;
        let lib: DataBase = serde_json::from_str(&content)?;
        Ok(lib)
    }
    pub async fn save_file(self, path: String) -> Result<(), DataBaseError> {
        let json_db = serde_json::to_string_pretty(&self)?;
        let r = std::fs::remove_file(&path);
        match r {
            Ok(_val) => (),
            _err => (),
        }
        let mut file = std::fs::File::create(&path)?;
        file.write_all(json_db.as_bytes())?;
        Ok(())
    }

    // Vec<Subject, Number of lates, Sum of lates, Percentage
    pub fn ranking_vec_lesson(&self, rank: Option<Ranking>) -> Vec<(Lesson, i32, u32, u32)> {
        let mut tupples: Vec<(Lesson, i32, u32, u32)> = Lesson::all()
            .into_iter()
            .map(|n: Lesson| (n.clone(), 0, self.sum_lesson(n), 0))
            .collect();
        for entry in &self.data {
            for t in &mut tupples {
                if entry.lesson == t.0 {
                    t.1 += 1;
                }
            }
        }
        for entry in &mut tupples {
            let percent = (entry.1 as f32 / self.data.len() as f32 * 100.0).round();
            entry.3 = percent as u32;
        }
        match rank {
            Some(ranking) => match ranking {
                Ranking::Number => {
                    tupples.sort_by(|e1, e2| e2.1.cmp(&e1.1));
                }

                Ranking::Sum => {
                    tupples.sort_by(|e1, e2| e2.2.cmp(&e1.2));
                }
            },
            None => {
                tupples.sort_by(|e1, e2| e2.1.cmp(&e1.1));
            }
        }
        tupples
    }

    // Vec<Person, Number of lates, Sum of lates, Percentage
    pub fn ranking_vec(&self, rank: Option<Ranking>) -> Vec<(Class, i32, u32, u32)> {
        let mut tupples: Vec<(Class, i32, u32, u32)> = Class::all()
            .into_iter()
            .map(|n: Class| (n.clone(), 0, self.sum_person(n), 0))
            .collect();
        for entry in &self.data {
            for t in &mut tupples {
                if entry.person == t.0 {
                    t.1 += 1;
                }
            }
        }
        for entry in &mut tupples {
            let percent = (entry.1 as f32 / self.data.len() as f32 * 100.0).round();
            entry.3 = percent as u32;
        }
        match rank {
            Some(ranking) => match ranking {
                Ranking::Number => {
                    tupples.sort_by(|e1, e2| e2.1.cmp(&e1.1));
                }

                Ranking::Sum => {
                    tupples.sort_by(|e1, e2| e2.2.cmp(&e1.2));
                }
            },
            None => {
                tupples.sort_by(|e1, e2| e2.1.cmp(&e1.1));
            }
        }
        tupples
    }

    pub fn get_percent_first_lesson(&self) -> i32 {
        let num: f32 = {
            let mut count = 0.0;
            for v in &self.data {
                if v.first_lesson {
                    count += 1.0;
                }
            }
            count
        };
        (num / self.data.len() as f32 * 100.0).round() as i32
    }

    pub fn average_delay(&self) -> f32 {
        if self.data.is_empty() {
            return 0.0;
        }
        let sum: u32 = self.data.iter().map(|x| x.delay_min).sum();
        sum as f32 / self.data.len() as f32
    }
    pub fn sum_min(&self) -> u32 {
        if self.data.is_empty() {
            return 0;
        }
        let sum: u32 = self.data.iter().map(|x| x.delay_min).sum();
        return sum;
    }
    fn sum_person(&self, person: Class) -> u32 {
        self.data
            .iter()
            .filter(|x| x.person == person)
            .map(|x| x.delay_min)
            .sum()
    }

    fn sum_lesson(&self, lesson: Lesson) -> u32 {
        self.data
            .iter()
            .filter(|x| x.lesson == lesson)
            .map(|x| x.delay_min)
            .sum()
    }

    pub fn penalties_person(&self, person: Class) -> u32 {
        self.entries_person_num(person) / 3
    }
    pub fn total_penalties(&self) -> u32 {
        Class::all()
            .into_iter()
            .map(|x| self.penalties_person(x))
            .sum()
    }

    fn entries_person_num(&self, person: Class) -> u32 {
        let mut n = 0;
        for entry in &self.data {
            if entry.person == person {
                n += 1;
            }
        }
        n
    }

    /*pub fn average_delay_time(&self) -> Vec<(Lesson, i32)> {
        let mut tupples: Vec<(Lesson, i32, i32)> = Lesson::all()
            .into_iter()
            .map(|n: Lesson| (n, 0, 0))
            .collect();
        for entry in &self.data {
            let entry_type = entry.lesson.clone();
            for l in &mut tupples {
                if l.0.eq(&entry_type) {
                    l.1 += entry.delay_min as i32;
                    l.2 += 1;
                    break;
                }
            }
        }
        for l in &mut tupples {
            if l.2 == 0 {
                continue;
            }
            l.1 /= l.2;
        }
        return tupples.into_iter().map(|n: Lesson, m, i| (n, m));
    }*/
    pub fn get_profile_stats(&self, person: Class) -> ProfileStats {
        let mut stats = ProfileStats::empty(person.clone());
        stats.sum = self.sum_person(person.clone());
        stats.num = self.entries_person_num(person.clone());
        stats.theo_penalties = self.penalties_person(person.clone());
        let mut num_first = 0;
        for entry in &self.data {
            if entry.person == person {
                if entry.delay_min < stats.min || stats.min == 0 {
                    stats.min = entry.delay_min;
                }
                if entry.delay_min > stats.max {
                    stats.max = entry.delay_min;
                }
                if entry.first_lesson {
                    num_first += 1;
                }
                match stats.latest {
                    None => stats.latest = Some(entry.clone()),
                    Some(ref e) => {
                        if e.date < entry.date {
                            stats.latest = Some(entry.clone());
                        } else if e.date == entry.date {
                            if e.lesson_time < entry.lesson_time {
                                stats.latest = Some(entry.clone());
                            }
                        }
                    }
                }
            }
        }
        stats.first_lesson_percent = num_first as f32 / stats.num as f32 * 100.0;
        stats.percent = (stats.num as f32 / self.data.len() as f32) * 100.0;
        stats.avg_min = stats.sum as f32 / stats.num as f32;
        stats
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Class {
    Nicole,
    Sophia,
    Emily,
    Leander,
    Suya,
    Jan,
    Liam,
    Evelin,
    Isabelle,
    Anina,
    Carlo,
    Levin,
    Anne,
    Kilian,
    Jonah,
    Ida,
    Neda,
    Antonie,
    Laurin,
    Marija,
    Raphael,
    Elena,
    Luis,
    Mia,
    Marie,
}

impl Class {
    pub fn all() -> Vec<Class> {
        let mut a = vec![
            Class::Nicole,
            Class::Emily,
            Class::Leander,
            Class::Suya,
            Class::Jan,
            Class::Liam,
            Class::Evelin,
            Class::Isabelle,
            Class::Anina,
            Class::Carlo,
            Class::Levin,
            Class::Anne,
            Class::Kilian,
            Class::Jonah,
            Class::Ida,
            Class::Neda,
            Class::Antonie,
            Class::Laurin,
            Class::Marija,
            Class::Raphael,
            Class::Elena,
            Class::Luis,
            Class::Mia,
            Class::Marie,
            Class::Sophia,
        ];
        a.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        a
    }
}

impl std::fmt::Display for Lesson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Lesson {
    Mathe,
    Bio,
    Deutsch,
    Physik,
    Franzoesisch,
    Italienisch,
    Geschichte,
    Englisch,
    Chemie,
    Sport,
    BG,
    Musik,
    Griechisch,
    Geographie,
}

impl Lesson {
    pub fn all() -> Vec<Lesson> {
        vec![
            Lesson::Mathe,
            Lesson::Bio,
            Lesson::Deutsch,
            Lesson::Physik,
            Lesson::Franzoesisch,
            Lesson::Italienisch,
            Lesson::Geschichte,
            Lesson::Englisch,
            Lesson::Chemie,
            Lesson::Sport,
            Lesson::BG,
            Lesson::Musik,
            Lesson::Griechisch,
            Lesson::Geographie,
        ]
    }
}
