use regex::{Regex,Captures};

#[derive(Debug,Serialize, Deserialize)]
pub struct LineItem {
  pub key:String,
  pub section:String,
  pub mode:String,
  pub values:Vec<f64>
}

#[derive(Debug,Serialize, Deserialize)]
pub struct KeyParts {
  pub key:String,
  pub section:String,
  pub mode:String
}


pub fn clean_line(line:&str) -> LineItem {
  let mut parts = line.trim().split(":");
  let first = parts.next().unwrap().to_owned();
  let key:String = first.to_string();
  let mut key_parts = KeyParts{
    key:key,
    section:"-".to_string(),
    mode:"-".to_string()
  };
  let re = Regex::new(r"^(\w+)([.-])(\w+)$").unwrap();
  let cap = re.captures(first.as_str());
  let val_str = parts.next().unwrap().to_owned();
  let vals:Vec<f64> = val_str.split(",").map(|s| value_string_to_f64(s)).collect();
  
  match cap {
    Some(matches) => {
      key_parts = match_key_parts(matches,first.as_str());
    },
    None => ()
  }

  LineItem {
    key: key_parts.key,
    section: key_parts.section,
    mode: key_parts.mode,
    values: vals
  }
}

pub fn match_key_parts(matches:Captures,key:&str) -> KeyParts {
  let mut str_key:&str = key;
  let mut str_mode:&str = "-"; 
  let mut str_section:&str = "-";
  match matches.get(2) {
    Some(m1) => {
      match m1.as_str() {
        "-" => {
          match matches.get(1) {
            Some(m2) => {
              if m2.as_str() == "ay" {
                str_section = "ayanamsas";
                str_mode = matches.get(3).unwrap().as_str();
              } else {
                str_section = "houses";
                str_mode = m2.as_str();
                str_key = matches.get(3).unwrap().as_str();
              }
            },
            _ => ()
          }
        },
        "." => {
          str_section = "bodies";
          str_mode = matches.get(1).unwrap().as_str();
          str_key = matches.get(3).unwrap().as_str();
        },
        _ => ()
      }
    },
    _ => ()
  }
  KeyParts{
    key:str_key.to_string(),
    section:str_section.to_string(),
    mode:str_mode.to_string()
  }
}

pub fn dms_string_to_decimal(dms:&str) -> f64 {
  let re = Regex::new(r"(?x)
      (?P<d>\d{1,3}) # degrees
      [^0-9]?\s*
      (?P<m>\d{1,2}) # minutes
      [^0-9]?\s*
      (?P<s>\d{1,2}(\.\d+)) # seconds
      ").unwrap();

    let cap = re.captures(dms);
    match cap {
      Some(matches) => dms_matches_to_degrees(matches),
      None => 0.0
    }
}

pub fn dms_matches_to_degrees(matches:Captures) -> f64 {
  let mut degrees:f64;
  match matches.name("d") {
    Some(m) => degrees = m.as_str().parse::<f64>().unwrap(),
    None => degrees = 0.0
  }
  match matches.name("m") {
    Some(m) => degrees += m.as_str().parse::<f64>().unwrap() / 60.0,
    None => degrees += 0.0
  }
  match matches.name("s") {
    Some(m) => degrees += m.as_str().parse::<f64>().unwrap() / 3600.0,
    None => degrees += 0.0
  }
  degrees
}

#[derive(Debug)]
pub struct EuroDateString {
  pub date:String,
  pub time:String
}

#[derive(Debug)]
pub struct CoordinatesString {
  pub lat:String,
  pub lng:String,
  pub alt:String
}

pub fn iso_datetime_to_euro(datestr:&str) -> EuroDateString {
  let mut parts = datestr.split("T");
  let date = parts.next().unwrap().to_owned();
  let time = parts.next().unwrap().to_owned();
  let dt:Vec<&str> = date.split("-").collect();
  let ti:Vec<&str> = time.split(":").collect();
  EuroDateString {
    date: vec![dt[2],dt[1],dt[0]].join(".").to_string(),
    time: vec![ti[0],ti[1]].join(".").to_string() + ti[2],
  }
}

pub fn comma_str_to_coords(coords:&str) -> CoordinatesString {
  let mut parts = coords.split(",");
  CoordinatesString {
    lat: parts.next().unwrap().to_owned(),
    lng: parts.next().unwrap().to_owned(),
    alt: parts.next().unwrap().to_owned()
  } 
}

pub trait DmsToDec {
  fn dms_to_decimal(&self) -> f64;
}

impl DmsToDec for str {
  fn dms_to_decimal(&self) -> f64 {
    dms_string_to_decimal(&self)
  }
}

pub fn value_string_to_f64(item:&str) -> f64 {
  let re = Regex::new(r"(?x)
      (?P<d>\d{1,3}) # degrees
      [^0-9]?\s*
      (?P<m>\d{1,2}) # minutes
      [^0-9]?\s*
      (?P<s>\d{1,2}(\.\d+)) # seconds
      ").unwrap();

    let cap = re.captures(item);
    match cap {
      Some(matches) => dms_matches_to_degrees(matches),
      None => {
        let out = item.parse::<f64>();
        if out.is_err() {
          0.0
        } else {
          out.unwrap()
        }
      }
    } 
}