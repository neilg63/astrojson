#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate grabinput;
extern crate chrono;
extern crate regex;

use std::collections::HashMap;
// use std::process::Command;
use std::env;

mod swetest;
mod sweph;
mod astro;

fn main() {
  let mut in_args = env::args();

  /*let mut coords:sweph::CoordinatesString = sweph::CoordinatesString{
    lat:"0.0".to_string(),
    lng:"0.0".to_string(),
    alt:"0.0".to_string()
  };*/
  if in_args.len() > 2 {
    //let cmd = in_args.nth(1).unwrap().to_owned();
    let datetime = in_args.nth(1).unwrap().to_owned();
    let coord_str = in_args.next().unwrap().to_owned();
    //let euro_date:sweph::EuroDateString = sweph::iso_datetime_to_euro(datetime.as_str());
    /*if coord_str.contains(",") {
      coords = sweph::comma_str_to_coords(coord_str.as_str());
    }*/

    /*let args = [euro_date.date,
      euro_date.time,
      coords.lng,
      coords.lat,
      coords.alt];*/

    /*let output = Command::new(cmd)
      .args(&args)
      .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });*/
    //let txt:String = String::from_utf8_lossy(&output.stdout).into_owned();

    let txt:String = swetest::fetch(datetime.as_str(), coord_str.as_str());
    let astro_data:astro::AstroData = output_to_astro_data(txt);
    let serialized = serde_json::to_string(&astro_data).unwrap();
    println!("{}",serialized);
  }

}

fn output_to_astro_data(txt:String) -> astro::AstroData {
  
   let mut geo_bodies:HashMap<String,Vec<f64>> = HashMap::new();
   let mut topo_bodies:HashMap<String,Vec<f64>> = HashMap::new();
   let mut f64_map:HashMap<String,f64> = HashMap::new();
   let mut coords_map:HashMap<String,astro::AstroObject> = HashMap::new();
   let mut houses:HashMap<String,Vec<f64>> = HashMap::new();
   let mut ayanamsas:HashMap<String,f64> = HashMap::new();

  let raw_lines = txt.lines();
  for line in raw_lines {
    if line.contains(":") && !line.ends_with(":") && line.len() > 5 {
      let li = sweph::clean_line(line);
      if li.mode == "g" {
        geo_bodies.insert(li.key,li.values);
      } else {
        match li.section.as_str() {
          "bodies" => {
            topo_bodies.insert(li.key,li.values);
          },
          "ayanamsas" => {
            ayanamsas.insert(li.mode,li.values.get(0).unwrap().to_owned());
          },
          "houses" => {
            if houses.contains_key(li.mode.as_str()) {
              let mut vals:Vec<f64> = houses.get(li.mode.as_str()).unwrap().to_owned();
              let nv = li.values.get(0).unwrap().to_owned();
               vals.push(nv);
              houses.insert(li.mode,vals);
            } else {
              if li.mode == "A" {
                f64_map.insert("ascendant".to_string(),li.values.get(0).unwrap().to_owned());
              }
              houses.insert(li.mode,li.values);
            } 
            
          },
          _ => {
            match li.key.as_str() {
              "ut" => {
                f64_map.insert("ut".to_string(),li.values.get(0).unwrap().to_owned());
                f64_map.insert("delta_t".to_string(),li.values.get(1).unwrap().to_owned());
              },
              "et" | "epsilon_true" | "vertex" | "mc" | "armc" | "ascendent" => {
                f64_map.insert(li.key,li.values.get(0).unwrap().to_owned());
              },
              "mean_node" |"true_node" |"mean_apogee" | "osc_apogee" |
              "intp_apogee" | "intp_perigee" | "nutation" => {
               if li.key.as_str() == "true_node" {
                  //let vals = ;
                  astro::add_rahuketu(&mut topo_bodies,&li.values);
                } else if li.key.as_str() == "mean_node" {
                  astro::add_rahuketu(&mut geo_bodies,&li.values);
                }
                coords_map.insert(li.key,astro::AstroObject::new(li.values));
              },
              _ => ()
            }
          }
        }
      }
    } 
  }
  let mut bodies:HashMap<String,astro::AstroObject> = HashMap::new();

  let mut body_lngs:HashMap<String,f64> = HashMap::new();
  let body_names:Vec<&str> = astro::body_names();
  for k in body_names {
    let bn = k.to_string();
    let vals = topo_bodies.get(k).unwrap().to_owned();
    if geo_bodies.contains_key(k) {
      let slice = vals.to_vec().as_slice().to_owned();
      body_lngs.insert(bn.clone(),slice[0]);
      let geo = geo_bodies.get(k).unwrap().to_owned();
      bodies.insert(k.to_string(),astro::AstroObject::new_body(vals,geo));
    } else {
      bodies.insert(k.to_string(),astro::AstroObject::new(vals));
    }
  }
  let fm = f64_map.clone();
  if fm.get("ascendant") != None {
    body_lngs.insert("ascendant".to_string(), *fm.get("ascendant").unwrap() );
  }
  if fm.get("mc") != None {
    body_lngs.insert("mc".to_string(), *fm.get("mc").unwrap() );
  } 
 
  let aspect_bands = astro::add_aspects(body_lngs);

  astro::AstroData{
    values: f64_map,
    coords: coords_map,
    bodies: bodies,
    houses: houses,
    ayanamsas: ayanamsas,
    aspects:aspect_bands
  }
}

