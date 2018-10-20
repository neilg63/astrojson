use std::process::{Command};
use regex::Regex;
use std::str::FromStr;

pub fn fetch(datetime:&str, location: &str) -> String {

  let mut lines:Vec<String> = query_swetest(datetime, location, "topo", "", vec!["-pa"] );
  
  lines.extend(query_swetest(datetime,location, "topo",  "sedna", vec!["-ps","-xs90377"] ));

  lines.extend(query_swetest(datetime,location, "geopos",  "", vec![] ));
  
  let houses:Vec<&str> = vec!["W","E","O","P","K","B","C","M","R","T","A","X","G","H"]; 
  lines.extend(query_swetest(datetime,location, "house_bearing",  "W", vec!["-p"] ));

  for h in houses {
    lines.extend(query_swetest(datetime,location, "house",  h, vec!["-p"] ));
  }
  let ayanamsas:Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 16, 21, 22, 23, 25, 26]; 
  
  
  for a in ayanamsas {
    let key = format!("ay-{}", a);
    let extra_arg = format!("-ay{}", a);
    lines.extend(query_swetest(datetime,"-", "ayanamsa",  key.as_str(), vec![extra_arg.as_str()] ));
  }
  let mut filter_lines:Vec<String> = lines.into_iter().filter(|s| filter_line(s)).collect();
  filter_lines.sort_unstable();
  filter_lines.dedup();
  filter_lines.join("\n")
}


fn query_swetest(datetime:&str,location:&str, mode:&str, extra_str:&str, extra_args:Vec<&str>) -> Vec<String> {
  let (date_str, time_str) = build_datetime_pairs(datetime);
  let mut loc_str:String = "".to_string();
  let mut args:Vec<&str> = vec![date_str.as_str(), time_str.as_str()];

  let filter_str = match mode {
    "house" => "house",
    "house_bearing" => "bearing",
    "ayanamsa" => "ayanamsa",
    _ => if extra_str.len() > 2 && !extra_str.contains("-") { extra_str } else { "" },
  };

  let prepend_str = match mode {
    "topo" => "t.",
    "geopos" => "g.",
    _ => ""
  };

  let add_location = match mode {
    "ayanamsa" => false,
    _ => true
  };
  
  if add_location {
    loc_str = build_location_str(location, mode, extra_str);
    args.push("-fPLBES");
    args.push(loc_str.as_ref());
  }
  let extra_letters = match mode {
    "topo" | "geopos" => "",
    _ => extra_str
  };
  args.extend(extra_args);
  let result = Command::new("swetest")
        .args(&args)
        .output()
        .expect("swetest command faile");
  let out = String::from_utf8_lossy(&result.stdout);
  let lines:Vec<String> = out.split("\n")
    .map(|s| s.to_lowercase())
    .filter(|s| match_line(s, filter_str))
    .map(|s| clean_line(s.to_string(),extra_letters, prepend_str))
    .collect();
  lines
}

fn clean_line(s:String,extra_str:&str, prepend:&str) -> String {
  let mut s2 = s.clone();
  s2 = fuzzy_replace(s2, r"[)(,]", "");
  s2 = fuzzy_replace(s2, r"([a-z])\.", "$1");
  if extra_str.len() > 0 {
    if s.contains("house") {
      let mut key = extra_str.to_string();
      key.push_str("-$1:");
      s2 = fuzzy_replace(s2, r"^house\s+(\d+)\b", key.as_str());
    } else if s.contains("ayanamsa") {
      s2 = fuzzy_replace(s2, r"ayanamsa\b", extra_str);
    }
  }
  s2 = fuzzy_replace(s2, r"(\d[°'])\s+(-?\d)", "$1$2");

  //s2 = fuzzy_replace(s2, r"^([^:]+):?\s+(-?\d)", "$1:$2");
  if extra_str.len() < 1 {
    s2 = fuzzy_replace(s2, r"^([a-z][a-z0-9-]+)\b\s+(-?\d)", "$1:$2");
    s2 = fuzzy_replace(s2, r"^([a-z][a-z0-9-]+)\s+([a-z][a-z0-9-]+)\b\s+(-?\d)", "$1 $2:$3");
    s2 = fuzzy_replace(s2, r"\s+:", ":");
    s2 = fuzzy_replace(s2, r"^([a-z]\w+)\s+([a-z]\w+)\s+:", "$1 $2:");
  } else {
    s2 = fuzzy_replace(s2, r"^([a-z][a-z0-9-]+)\s+(-?\d)", "$1:$2");
    s2 = fuzzy_replace(s2, r"^([a-z][a-z0-9-]+)\s+:", "$1:");
  }
  if extra_str.len() < 1 {
    s2 = fuzzy_replace(s2, r"(\d)\s+(-?\d)", "$1,$2");
    s2 = fuzzy_replace(s2, r"\s+delta\s+t:\s+",",");
    s2 = fuzzy_replace(s2, r"\s+sec\s*$","");
  }
  s2 = fuzzy_replace(s2, r"\s+(-?\d)", "$1");
  s2 = fuzzy_replace(s2.trim().to_string(), r"\s+","_");
  if prepend.len() > 0 {
    if !s2.starts_with("ut:") && 
      !s2.starts_with("et:") && 
      !s2.starts_with("epsilon_true:") && 
      !s2.starts_with("nutation:") &&
      !s2.starts_with("ascendant:") &&
      !s2.starts_with("mc:") &&
      !s2.starts_with("mean_node:") &&
      !s2.starts_with("true_node:") &&
      !s2.starts_with("mean_apogee:") &&
      !s2.starts_with("osc_apogee:") &&
      !s2.starts_with("intp_apogee:") &&
      !s2.starts_with("intp_perigee:") &&
      !s2.starts_with("vertex:") &&
      !s2.starts_with("armc:") {
      s2.insert_str(0, prepend);
    }
  }
  s2 = fuzzy_replace(s2, r"^([a-z][a-z0-9-]+)_(-?\d)", "$1:$2");
  s2
}

fn build_location_str(location:&str, prefix:&str, suffix:&str) -> String {
  let loc_parts:Vec<&str> = location.split(",").collect();
  let mut loc_str:String = "-".to_string();
  loc_str.push_str(prefix);
  loc_str.push_str(loc_parts[1]);
  loc_str.push_str(",");
  loc_str.push_str(loc_parts[0]);
  loc_str.push_str(",");
  if prefix == "house" || prefix == "house_bearing" {
    if suffix.len() > 0 {
      loc_str.push_str(suffix);
    } else {
      loc_str.push_str("W");
    }
  } else {
    if loc_parts.len() > 2 {
      loc_str.push_str(loc_parts[2]);
    } else {
      loc_str.push_str("30");
    } 
  }
  loc_str
}

fn build_datetime_pairs(datetime:&str) -> (String, String) {
  let datetime_parts:Vec<&str> = datetime.split("T").collect();
  let date_part = datetime_parts[0];
  let time_part = datetime_parts[1];
  let date_parts:Vec<&str> = date_part.split("-").collect();
  let mut date_str = "-b".to_string();
  
  date_str.push_str(date_parts[2]);
  date_str.push_str(".");
  date_str.push_str(date_parts[1]);
  date_str.push_str(".");
  date_str.push_str(date_parts[0]);
  let mut time_str = "-ut".to_string();
  time_str.push_str(time_part);
  (date_str, time_str)
}

fn filter_line(s:&str) -> bool {
  let mut valid = s.len() > 5 && s.contains(":");
  if valid {
    if !s.contains("ay-") && !s.contains(",") && Some(1) == s.find('-') {
      let parts:Vec<&str> = s.split(":").collect();
      let sub_parts:Vec<&str> = parts[0].split("-").collect();
      if sub_parts.len() > 0 {
        let n = u16::from_str(sub_parts[1]).expect("invalid integer");
        valid = match sub_parts[0] {
          "G" => n <= 18,
          _ =>n <= 6
        };
      }
    }
  }
  valid
}

fn fuzzy_replace(s:String, regex:&str, replacement: &str) -> String {
  let re = Regex::new(regex).unwrap();
  re.replace_all(s.as_str(), replacement).into_owned()
}

/*fn fuzzy_match(s:String, regex:&str) -> bool {
  let re = Regex::new(regex).unwrap();
  re.is_match(s.as_str())
}*/

fn match_line(s:&str, filter: &str) -> bool {
  let mut valid = !(
    s.starts_with("swetest") ||
    s.contains("version") ||
    s.contains("whole") ||
    s.contains("equal") ||
    s.contains("geo. long") ||
    (s.contains("house") && s.contains("system"))
  );
  if valid && filter.len() > 0 {
    if filter == "bearing" {
      valid = s.starts_with("ascendant") ||
        s.contains("mc") ||
        s.contains("armc") ||
        s.contains("vertex")
    } else {
      valid = s.contains(filter)
    }
  }
  valid
}