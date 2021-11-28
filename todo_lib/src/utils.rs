use crate::Date;
use serde::{Deserialize, Serialize};


pub const PROJECT_TAG: &str = "+"; // project marker
pub const CONTEXT_TAG: &str = "@"; // context marker
pub const HASH_TAG: &str = "#"; // hashtag marker
// pub const NO_PRIORITY: u8 = 26; // the 26th letter of the alphabet
// pub const DUE_TAG: &str = "due"; // Due by
// pub const THR_TAG: &str = "t"; // Threshold or Start date
// pub const REC_TAG: &str = "rec"; // Reoccurence tag
pub const DUE_TAG_FULL: &str = "due:";
// pub const THR_TAG_FULL: &str = "t:";
pub const REC_TAG_FULL: &str = "rec:";
pub const CREATOR_TAG_FULL: &str = "cre:"; // task created by
pub const ASSIGNED_TAG_FULL: &str = "resp:"; // task reponsible


#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize, Eq, Ord, PartialOrd)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize, Eq, Ord, PartialOrd)]
pub struct Recurrence {
    pub period: Period,
    pub count: u8,
    pub strict: bool,
}

impl Default for Recurrence {
    fn default() -> Self {
        Recurrence {
            period: Period::Day,
            count: 0,
            strict: false,
        }
    }
}

impl std::str::FromStr for Recurrence {
    type Err = String;
    fn from_str(s: &str) -> Result<Recurrence, String> {
        Recurrence::parse(&s.to_owned())
    }
}

impl std::fmt::Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(REC_TAG_FULL)?;
        if self.strict {
            f.write_str("+")?;
        }
        f.write_fmt(format_args!("{}", self.count))?;
        match self.period {
            Period::Day => f.write_str("d"),
            Period::Week => f.write_str("w"),
            Period::Month => f.write_str("m"),
            Period::Year => f.write_str("y"),
        }
    }
}

impl Recurrence {
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = if let Some(stripped) = s.strip_prefix(REC_TAG_FULL) {
            stripped
        } else {
            s
        };
        let mut rec = Recurrence::default();
        if s.ends_with('d') {
            rec.period = Period::Day;
        } else if s.ends_with('w') {
            rec.period = Period::Week;
        } else if s.ends_with('m') {
            rec.period = Period::Month;
        } else if s.ends_with('y') {
            rec.period = Period::Year;
        } else {
            return Err(format!("invalid recurrence '{}'", s));
        }
        if s.starts_with('+') {
            rec.strict = true;
        }
        let num = s[..s.len() - 1].parse::<u8>();
        match num {
            Err(_) => Err(format!("invalid recurrence '{}'", s)),
            Ok(n) => {
                rec.count = n;
                Ok(rec)
            }
        }
    }
}

pub fn days_in_month(y: u32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => {
            if y % 4 == 0 {
                if y % 100 == 0 && y % 400 != 0 {
                    28
                } else {
                    29
                }
            } else {
                28
            }
        }
        _ => 30,
    }
}

pub fn try_read_date(input: &str) -> Option<Date> {
    // if there are 2 dashes in the 10 char length then we espect a date
    let target = String::from_utf8_lossy(&input.as_bytes()[..11]);
    if target.matches("-").count() == 2 {
        let mut values = vec![];
        for section in target.split("-") {
            match section.parse::<u32>() {
                Err(_) => return None,
                Ok(value) => values.push(value),
            }
        }
        // Check for other not valid dates
        if values.len() != 3 {
            println!("invalid date '{}'", input);
            return None
        }
        if values[0] == 0 {
            println!("invalid year '{}'", input);
            return None
        }
        if values[1] == 0 || values[1] > 12 {
            println!("invalid month '{}'", input);
            return None
        }
        if values[2] == 0 || values[2] > days_in_month(values[0], values[1]) {
            println!("invalid day '{}'", input);
            return None
        }
        Some(Date::from_ymd(values[0] as i32, values[1], values[2]))
    } else {
        None
    }
}

/// Converts a string like "(A)" to a u8 up to 26
pub fn parse_priority(s: &str) -> Result<u8, String> {
    if s.len() != 3 {
        return Err(format!("invalid priority '{}'", s));
    }
    // removes all the characters not below
    let trimmed = s.trim_matches(|c| c == ' ' || c == '(' || c == ')');
    if trimmed.len() != 1 {
        return Err(format!("invalid priority '{}'", s));
    }

    let priority = trimmed.bytes().next().expect("impossible");
    // if not a capital letter
    if !(b'A'..=b'Z').contains(&priority) {
        return Err(format!("invalid priority '{}'", s));
    }
    // adjusted byte as u8
    Ok((priority - b'A') as u8)
}

pub fn next_word(s: &str) -> &str {
    if s.is_empty() {
        return s;
    }
    match s.find(' ') {
        None => s,
        Some(p) => &s[..p],
    }
}
