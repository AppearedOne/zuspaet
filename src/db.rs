use crate::time::{self, get_today};
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
pub struct DataBase {
    pub data: Vec<Entry>,
}
impl DataBase {
    pub fn empty() -> DataBase {
        DataBase { data: vec![] }
    }
    pub async fn load_file(path: &str) -> DataBase {
        let content = std::fs::read_to_string(path).expect("couldnt read");
        let lib: DataBase = serde_json::from_str(&content).expect("Couldnt parse");
        lib
    }
    pub async fn save_file(self, path: String) -> Result<(), DataBaseError> {
        let json_db = serde_json::to_string_pretty(&self)?;
        let r = std::fs::remove_file(&path);
        match r {
            Ok(_val) => (),
            _err => (),
        }
        let mut file = std::fs::File::create(&path)?;
        file.write(json_db.as_bytes())?;
        Ok(())
    }
    pub fn ranking_vec(&self) -> Vec<(Class, i32)> {
        let mut tupples: Vec<(Class, i32)> =
            Class::all().into_iter().map(|n: Class| (n, 0)).collect();
        for entry in &self.data {
            for t in &mut tupples {
                if entry.person == t.0 {
                    t.1 += 1;
                }
            }
        }
        tupples.sort_by(|e1, e2| e2.1.cmp(&e1.1));
        tupples
    }

    pub fn average_delay(&self) -> f32 {
        if self.data.is_empty() {
            return 0.0;
        }
        let sum: u32 = self.data.iter().map(|x| x.delay_min).sum();
        sum as f32 / self.data.len() as f32
    }
    pub fn ranking_vec_lesson(&self) -> Vec<(Lesson, i32)> {
        let mut tupples: Vec<(Lesson, i32)> =
            Lesson::all().into_iter().map(|n: Lesson| (n, 0)).collect();
        for entry in &self.data {
            for t in &mut tupples {
                if entry.lesson == t.0 {
                    t.1 += 1;
                }
            }
        }
        tupples.sort_by(|e1, e2| e2.1.cmp(&e1.1));
        tupples
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
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Class {
    Nicole,
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
        vec![
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
        ]
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
