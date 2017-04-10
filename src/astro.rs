use std::collections::HashMap;

pub fn add_aspects(body_lngs:HashMap<String,f64>) -> AspectBands{
    let mut opposition: Vec<Aspect> = Vec::new();
  let mut triseptile: Vec<Aspect> = Vec::new();
  let mut quadnovile: Vec<Aspect> = Vec::new();
  let mut inconjunction: Vec<Aspect> = Vec::new();
  let mut biquintile: Vec<Aspect> = Vec::new();
  let mut sesquisquare: Vec<Aspect> = Vec::new();
  let mut trine: Vec<Aspect> = Vec::new();
  let mut tridecile: Vec<Aspect> = Vec::new();
  let mut biseptile: Vec<Aspect> = Vec::new();
  let mut square: Vec<Aspect> = Vec::new();
  let mut binovile: Vec<Aspect> = Vec::new();
  let mut quintile: Vec<Aspect> = Vec::new();
  let mut sextile: Vec<Aspect> = Vec::new();
  let mut septile: Vec<Aspect> = Vec::new();
  let mut semisquare: Vec<Aspect> = Vec::new();
  let mut novile: Vec<Aspect> = Vec::new();
  let mut dectile: Vec<Aspect> = Vec::new();
  let mut semisextile: Vec<Aspect> = Vec::new();
  let mut conjunction: Vec<Aspect> = Vec::new();
  let mut froms:Vec<String> = Vec::new();
  let pc = body_lngs.clone();


  for (bn,lng) in body_lngs {
    let p2 = pc.clone();

    for (bn2,lng2) in p2 {
      let mkr = bn2.clone() + "|" + bn.as_str();
      if froms.binary_search(&mkr).is_err() && bn.as_str() != bn2.as_str() {
        let mk = bn.clone() + "|" + bn2.as_str();
        if is_within_aspect(lng2,lng,180.0,bn.clone(),bn2.clone(),"opposition") {
          opposition.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,160.0,bn.clone(),bn2.clone(),"quadnovile") {
          quadnovile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,(360.0/7.0*3.0),bn.clone(),bn2.clone(),"triseptile") {
          triseptile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,150.0,bn.clone(),bn2.clone(),"inconjunction") {
          inconjunction.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,144.0,bn.clone(),bn2.clone(),"sesquisquare") {
          sesquisquare.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,135.0,bn.clone(),bn2.clone(),"biquintile") {
          biquintile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,120.0,bn.clone(),bn2.clone(),"trine") {
          trine.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,108.0,bn.clone(),bn2.clone(),"tridecile") {
          tridecile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,(360.0/3.5),bn.clone(),bn2.clone(),"biseptile") {
          biseptile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,90.0,bn.clone(),bn2.clone(),"square") {
          square.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,80.0,bn.clone(),bn2.clone(),"binovile") {
          binovile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,72.0,bn.clone(),bn2.clone(),"quintile") {
          quintile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,60.0,bn.clone(),bn2.clone(),"sextile") {
          sextile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,(360.0/7.0),bn.clone(),bn2.clone(),"triseptile") {
          septile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,45.0,bn.clone(),bn2.clone(),"semisquare") {
          semisquare.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,40.0,bn.clone(),bn2.clone(),"novile") {
          novile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,36.0,bn.clone(),bn2.clone(),"dectile") {
          dectile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,30.0,bn.clone(),bn2.clone(),"semisextile") {
          semisextile.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        } else if is_within_aspect(lng2,lng,0.0,bn.clone(),bn2.clone(),"conjunction") {
          conjunction.push(Aspect{
            from: bn.to_string(),
            to: bn2.to_string(),
            start: lng,
            end: lng2
          });
          froms.push(mk);
        }
      }
    }
  }

  AspectBands {
    opposition: opposition,
    quadnovile: quadnovile,
    triseptile: triseptile,
    inconjunction: inconjunction,
    biquintile: biquintile,
    sesquisquare: sesquisquare,
    trine: trine,
    tridecile: tridecile,
    biseptile: biseptile,
    square: square,
    binovile: binovile,
    quintile: quintile,
    sextile: sextile,
    semisquare: semisquare,
    septile: septile,
    novile: novile,
    dectile: dectile,
    semisextile: semisextile,
    conjunction: conjunction
  }
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
  pub aspects:AspectBands
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AspectBands {
  opposition: Vec<Aspect>,
  triseptile: Vec<Aspect>,
  quadnovile:Vec<Aspect>,
  inconjunction: Vec<Aspect>,
  biquintile: Vec<Aspect>,
  sesquisquare: Vec<Aspect>,
  trine: Vec<Aspect>,
  tridecile:Vec<Aspect>,
  biseptile: Vec<Aspect>,
  square: Vec<Aspect>,
  binovile: Vec<Aspect>,
  quintile: Vec<Aspect>,
  sextile: Vec<Aspect>,
  septile: Vec<Aspect>,
  semisquare: Vec<Aspect>,
  novile: Vec<Aspect>,
  dectile: Vec<Aspect>,
  semisextile: Vec<Aspect>,
  conjunction: Vec<Aspect>
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Aspect {
  from:String,
  to:String,
  start: f64,
  end: f64
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
    "juno"]
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

fn is_within_aspect(val:f64, val2:f64, angle:f64,b1:String,b2:String,aspect:&str) -> bool {
  let target = (val2+angle)%360.0;
  let orb = calc_orb(b1, b2,aspect);
  is_in_range(val,target,orb)
}

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