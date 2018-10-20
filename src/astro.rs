use std::collections::HashMap;

pub fn add_aspects(body_lngs:HashMap<String,f64>) -> Vec<AspectData>{
  //let mut ab = AspectBands::new();
  let mut froms:Vec<String> = Vec::new();
  let pc = body_lngs.clone();

  let mut aspect_groups = aspect_groups();

  let num_aspect_groups = aspect_groups.len();

  for (bn,lng) in body_lngs {
    let p2 = pc.clone();

    for (bn2,lng2) in p2 {
      let mkr = bn2.clone() + "|" + bn.as_str();
      if froms.binary_search(&mkr).is_err() && bn.as_str() != bn2.as_str() {
        
        for i in 0..num_aspect_groups {
          let mk = bn.clone() + "|" + bn2.as_str();
          let target = (lng2+aspect_groups[i].angle)%360.0;
          let orb = calc_orb(bn.clone(), bn2.clone(),aspect_groups[i].name.as_str());
          if is_in_range(lng,target,orb) {
            aspect_groups[i].items.push(Aspect{
              from: bn.clone(),
              to:  bn2.clone(),
              start: lng,
              end: lng2,
              orb: orb
            });
         
            froms.push(mk);
          }
        }
      }
    }
  }
  aspect_groups
}


fn calc_orb(b1:String, b2:String,aspect:&str) -> f64 {
  let orbs:[[f64; 6]; 6] = [
    [9.0, 5.0,  1.5,  3.0,  1.0,  0.5],
    [7.0,  5.0,  1.5,  3.0,  1.0,  0.5],
    [8.0,  5.0,  1.5,  3.0,  1.0,  0.5],
    [3.0,  2.0,  1.0,  1.0,  1.0,  0.5],
    [1.0,  0.0,  0.0,  0.0,  0.0,  0.0],
    [7.0,  5.0,  1.5,  3.0,  1.0,  0.5]
  ];
  let b1_index:usize = calc_orb_body_group(b1.as_str());
  let b2_index:usize = calc_orb_body_group(b2.as_str());
  let aspect_index:usize = calc_orb_aspect_group(aspect);
  let orb1:f64 = orbs[aspect_index][b1_index];
  let orb2:f64 = orbs[aspect_index][b2_index];
  (orb1 + orb2) / 2.0
}

fn calc_orb_body_group(body:&str) -> usize {
  match body {
    "sun" | "moon"  => 0,
    "mercury" | "venus" | "mars" => 1,
    "jupiter" | "saturn" => 2,
    "uranus" | "neptune" | "pluto" => 3,
    "rahu" | "ketu" => 4,
    _ => 5
  }
}

fn calc_orb_aspect_group(aspect:&str) -> usize {
  match aspect {
    "conjunction" | "opposition"  | "trine" | "square" => 0,
    "sextile" => 1,
    "semisextile" => 2,
    "sesquisquare" | "inconjunction" | "semisquare" => 3,
    "quintile" | "biquintile" => 4,
    _ => 5
  }
}



#[derive(Debug,Serialize, Deserialize)]
pub struct AstroData {
  pub values: HashMap<String,f64>,
  pub coords:HashMap<String,AstroObject>,
  pub bodies:HashMap<String,AstroObject>,
  pub houses:HashMap<String,Vec<f64>>,
  pub ayanamsas:HashMap<String,f64>,
  pub aspects:Vec<AspectData>
}

pub fn aspect_groups() -> Vec<AspectData> {
  let mut ag:Vec<AspectData> = Vec::new();
  ag.push(AspectData::new("opposition",180.0));
  ag.push(AspectData::new("quadnovile",160.0));
  ag.push(AspectData::new("triseptile", 360.0/7.0*3.0 )); // 154.28571
  ag.push(AspectData::new("inconjunction",150.0));
  ag.push(AspectData::new("biquintile",135.0));
  ag.push(AspectData::new("sesquisquare",144.0));
  ag.push(AspectData::new("trine",120.0));
  ag.push(AspectData::new("tridecile",108.0));
  ag.push(AspectData::new("biseptile",360.0/3.5 )); // 102.85714
  ag.push(AspectData::new("square",90.0));
  ag.push(AspectData::new("binovile",80.0));
  ag.push(AspectData::new("quintile",72.0));
  ag.push(AspectData::new("sextile",60.0));
  ag.push(AspectData::new("septile", 360.0/7.0 )); // 51.42857
  ag.push(AspectData::new("semisquare",45.0));
  ag.push(AspectData::new("novile",40.0));
  ag.push(AspectData::new("dectile",36.0));
  ag.push(AspectData::new("semisextile",30.0));
  ag.push(AspectData::new("conjunction",0.0));
  ag
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AspectData {
  name: String,
  angle: f64,
  items: Vec<Aspect>
}

impl AspectData {
  pub fn new(name:&str,angle:f64) -> AspectData {
    AspectData {
      name: name.to_string(),
      angle: angle,
      items: Vec::new()
    }
  }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Aspect {
  from:String,
  to:String,
  start: f64,
  end: f64,
  orb: f64
}


#[derive(Debug,Serialize, Deserialize)]
#[serde(untagged)]
pub enum AstroObject {
  LatLng {
    lat:f64,
    lng:f64
  },
  Coords {
    lat:f64,
    lng:f64,
    spd:f64
  },
  Body {
    lat:f64,
    lng:f64,
    spd:f64,
    glat:f64,
    glng:f64,
    gspd:f64
  }
}

impl AstroObject {
  pub fn new(items:Vec<f64>) -> AstroObject {
    if items.len() == 3 {
      AstroObject::Coords{
        lat: items.get(1).unwrap().to_owned(),
        lng: items.get(0).unwrap().to_owned(),
        spd: items.get(2).unwrap().to_owned()
      }
    } else if items.len() == 2 {
      AstroObject::LatLng {
        lat: items.get(1).unwrap().to_owned(),
        lng: items.get(0).unwrap().to_owned()
      }
    } else {
      AstroObject::LatLng {
        lat: 0.0,
        lng: 0.0
      }
    }
  }

  pub fn new_body(items:Vec<f64>,geo_items:Vec<f64>) -> AstroObject {
    if items.len() == 3 && geo_items.len() == 3 {
      AstroObject::Body {
        lat: items.get(1).unwrap().to_owned(),
        lng: items.get(0).unwrap().to_owned(),
        spd: items.get(2).unwrap().to_owned(),
        glat: geo_items.get(1).unwrap().to_owned(),
        glng: geo_items.get(0).unwrap().to_owned(),
        gspd: geo_items.get(2).unwrap().to_owned()
      }
    } else {
      AstroObject::Body {
        lat:0.0,
        lng:0.0,
        spd:0.0,
        glat:0.0,
        glng:0.0,
        gspd:0.0
      }
    }
  }
}


pub fn body_names<'a>() -> Vec<&'a str> {
	vec!["sun",
    "moon",
    "mercury",
    "venus",
    "mars",
    "jupiter",
    "saturn",
    "uranus",
    "neptune",
    "pluto",
    "rahu",
    "ketu",
    "vesta",
    "ceres",
    "pholus",
    "chiron",
    "pallas",
    "juno",
    "sedna"]
}

pub fn add_rahuketu(bodies:&mut HashMap<String,Vec<f64>>,vals:&Vec<f64>) {
  bodies.insert("rahu".to_string(),vals.to_owned());
  let vals2 = vec![
    (vals.get(0).unwrap().to_owned() + 180.0) % 360.0,
    vals.get(1).unwrap().to_owned(),
    vals.get(2).unwrap().to_owned()
  ];
  bodies.insert("ketu".to_string(),vals2.to_owned());
}



fn is_between(val:f64, lower:f64,upper:f64) -> bool {
  if upper < lower {
    (val > lower || val <= upper)
  } else {
    (val < upper && val >= lower)
  }
}

/*fn is_within_angle(val:f64, val2:f64, angle:f64,tolerance:f64) -> bool {
  let target = (val2+angle)%360.0;
  is_in_range(val,target,tolerance)
}*/


/*fn is_within_aspect(val:f64, val2:f64, angle:f64,b1:String,b2:String,aspect:&str) -> bool {
  let target = (val2+angle)%360.0;
  let orb = calc_orb(b1, b2,aspect);
  is_in_range(val,target,orb)
}*/

/*fn match_aspect(mut aspect_vec:Vec<Aspect>,val:f64, val2:f64, angle:f64,b1:String,b2:String,aspect:&str) -> bool {
  let target = (val2+angle)%360.0;
  let orb = calc_orb(b1.clone(), b2.clone(),aspect);
  if is_in_range(val,target,orb) {
    aspect_vec.push(Aspect{
      from: b1,
      to: b2,
      start: val,
      end: val2
    });
    return true;
  }
  return false;
}*/

fn is_in_range(val:f64, target:f64,tolerance:f64) -> bool {
  let lower = target - tolerance;
  let upper = target + tolerance;
  is_between(val,lower,upper)
}