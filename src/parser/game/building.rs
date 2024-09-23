use derive_builder::Builder;
use nom::{bytes::complete::tag, IResult};

use super::super::{read_i64, read_next_length, read_string};

#[derive(Default, Builder, Debug, Clone)]
pub struct Building {
  pub id: i64,
  pub games_id: i64,
  pub terrain_id: i64,
  pub x: i64,
  pub y: i64,
  pub capture: i64,
  pub last_capture: i64,
  pub last_updated: String,
}

pub fn read_buildings(input: &str) -> IResult<&str, Vec<Building>> {
  // s:9:"buildings";
  let (input, _) = tag("a:")(input)?;
  let (input, length) = read_next_length(input)?;
  let (input, _) = tag("{")(input)?;

  let mut buildings = Vec::new();
  let mut input_loop = input;

  for _i in 0..length {
    let input = input_loop;
    let (input, _building_index) = read_i64(input)?;
    let (input, _) = tag(r#"O:12:"awbwBuilding":"#)(input)?;
    let (input, num_fields) = read_next_length(input)?;
    let (input, _) = tag("{")(input)?;
    let mut building_builder = BuildingBuilder::default();

    input_loop = input;
    for _j in 0..num_fields {
      let input = input_loop;
      let (input, entry) = read_string(input)?;
      let input = match entry {
        "id" => {
          let (input, value) = read_i64(input)?;
          building_builder.id(value);
          input
        }
        "games_id" => {
          let (input, value) = read_i64(input)?;
          building_builder.games_id(value);
          input
        }
        "terrain_id" => {
          let (input, value) = read_i64(input)?;
          building_builder.terrain_id(value);
          input
        }
        "x" => {
          let (input, value) = read_i64(input)?;
          building_builder.x(value);
          input
        }
        "y" => {
          let (input, value) = read_i64(input)?;
          building_builder.y(value);
          input
        }
        "capture" => {
          let (input, value) = read_i64(input)?;
          building_builder.capture(value);
          input
        }
        "last_capture" => {
          let (input, value) = read_i64(input)?;
          building_builder.last_capture(value);
          input
        }
        "last_updated" => {
          let (input, value) = read_string(input)?;
          building_builder.last_updated(value.to_owned());
          input
        }
        unknown => panic!("Replay player data contained unknown entry: {unknown}"),
      };
      input_loop = input;
    }
    buildings.push(building_builder.build().unwrap());

    let input = input_loop;
    let (input, _) = tag("}")(input)?;
    input_loop = input;
  }

  let input = input_loop;

  let (input, _) = tag("}")(input)?;
  Ok((input, buildings))
}
