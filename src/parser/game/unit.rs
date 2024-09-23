use derive_builder::Builder;
use nom::{bytes::complete::tag, IResult};

use super::super::{read_bool, read_f64, read_i64, read_next_length, read_string};

#[derive(Default, Builder, Debug, Clone)]
pub struct Unit {
  pub id: i64,
  pub games_id: i64,
  pub players_id: i64,
  pub name: String,
  pub movement_points: i64,
  pub vision: i64,
  pub fuel: i64,
  pub fuel_per_turn: i64,
  pub sub_dive: bool,
  pub ammo: i64,
  pub short_range: i64,
  pub long_range: i64,
  pub second_weapon: bool,
  pub symbol: String,
  pub cost: i64,
  pub movement_type: String,
  pub x: i64,
  pub y: i64,
  pub moved: i64,
  pub capture: i64,
  pub fired: i64,
  pub hit_points: f64,
  pub cargo1_units_id: i64,
  pub cargo2_units_id: i64,
  pub carried: bool,
}

pub fn read_units(input: &str) -> IResult<&str, Vec<Unit>> {
  // s:5:"units";
  let (input, _) = tag("a:")(input)?;
  let (input, length) = read_next_length(input)?;
  let (input, _) = tag("{")(input)?;

  let mut units = Vec::new();
  let mut input_loop = input;

  for _i in 0..length {
    let input = input_loop;
    let (input, _unit_index) = read_i64(input)?;
    let (input, _) = tag(r#"O:8:"awbwUnit":"#)(input)?;
    let (input, num_fields) = read_next_length(input)?;
    let (input, _) = tag("{")(input)?;
    let mut unit_builder = UnitBuilder::default();

    input_loop = input;
    for _j in 0..num_fields {
      let input = input_loop;
      let (input, entry) = read_string(input)?;
      let input = match entry {
        "id" => {
          let (input, value) = read_i64(input)?;
          unit_builder.id(value);
          input
        }
        "games_id" => {
          let (input, value) = read_i64(input)?;
          unit_builder.games_id(value);
          input
        }
        "players_id" => {
          let (input, value) = read_i64(input)?;
          unit_builder.players_id(value);
          input
        }
        "name" => {
          let (input, value) = read_string(input)?;
          unit_builder.name(value.to_owned());
          input
        }
        "movement_points" => {
          let (input, value) = read_i64(input)?;
          unit_builder.movement_points(value);
          input
        }
        "vision" => {
          let (input, value) = read_i64(input)?;
          unit_builder.vision(value);
          input
        }
        "fuel" => {
          let (input, value) = read_i64(input)?;
          unit_builder.fuel(value);
          input
        }
        "fuel_per_turn" => {
          let (input, value) = read_i64(input)?;
          unit_builder.fuel_per_turn(value);
          input
        }
        "sub_dive" => {
          let (input, value) = read_bool(input)?;
          unit_builder.sub_dive(value);
          input
        }
        "ammo" => {
          let (input, value) = read_i64(input)?;
          unit_builder.ammo(value);
          input
        }
        "short_range" => {
          let (input, value) = read_i64(input)?;
          unit_builder.short_range(value);
          input
        }
        "long_range" => {
          let (input, value) = read_i64(input)?;
          unit_builder.long_range(value);
          input
        }
        "second_weapon" => {
          let (input, value) = read_bool(input)?;
          unit_builder.second_weapon(value);
          input
        }
        "symbol" => {
          let (input, value) = read_string(input)?;
          unit_builder.symbol(value.to_owned());
          input
        }
        "cost" => {
          let (input, value) = read_i64(input)?;
          unit_builder.cost(value);
          input
        }
        "movement_type" => {
          let (input, value) = read_string(input)?;
          unit_builder.movement_type(value.to_owned());
          input
        }
        "x" => {
          let (input, value) = read_i64(input)?;
          unit_builder.x(value);
          input
        }
        "y" => {
          let (input, value) = read_i64(input)?;
          unit_builder.y(value);
          input
        }
        "moved" => {
          let (input, value) = read_i64(input)?;
          unit_builder.moved(value);
          input
        }
        "capture" => {
          let (input, value) = read_i64(input)?;
          unit_builder.capture(value);
          input
        }
        "fired" => {
          let (input, value) = read_i64(input)?;
          unit_builder.fired(value);
          input
        }
        "hit_points" => {
          let (input, value) = read_f64(input)?;
          unit_builder.hit_points(value);
          input
        }
        "cargo1_units_id" => {
          let (input, value) = read_i64(input)?;
          unit_builder.cargo1_units_id(value);
          input
        }
        "cargo2_units_id" => {
          let (input, value) = read_i64(input)?;
          unit_builder.cargo2_units_id(value);
          input
        }
        "carried" => {
          let (input, value) = read_bool(input)?;
          unit_builder.carried(value);
          input
        }
        unknown => panic!("Replay player data contained unknown entry: {unknown}"),
      };
      input_loop = input;
    }
    units.push(unit_builder.build().unwrap());

    let input = input_loop;
    let (input, _) = tag("}")(input)?;
    input_loop = input;
  }

  let input = input_loop;

  let (input, _) = tag("}")(input)?;
  Ok((input, units))
}
