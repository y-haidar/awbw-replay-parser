use derive_builder::Builder;
use nom::{bytes::complete::tag, IResult};

use crate::parser::{read_next_length, read_string};

use super::super::{read_bool, read_i64, read_i64_opt, read_string_opt};

// #[derive(Default, Debug, Clone)]
// pub enum CoPower {
//   #[default]
//   // N
//   None,
//   // Y
//   Normal,
//   // S
//   Super,
// }

#[derive(Default, Builder, Debug, Clone)]
pub struct Player {
  pub id: i64,
  pub user_id: i64,
  pub games_id: i64,
  pub countries_id: i64,
  pub co_id: i64,
  pub funds: i64,
  pub turn: Option<String>,
  pub email: Option<String>,
  pub uniq_id: Option<String>,
  pub eliminated: bool,
  // TODO: use datetime?
  pub last_read: Option<String>,
  pub last_read_broadcasts: Option<String>,
  pub emailpress: Option<String>,
  pub signature: Option<String>,
  pub co_power: i64,
  pub co_power_on: bool,
  pub order: i64,
  pub accept_draw: bool,
  pub co_max_power: Option<i64>,
  pub co_max_spower: Option<i64>,
  pub co_image: Option<String>,
  pub team: String,
  pub aet_count: i64,
  // TODO: use datetime?
  pub turn_start: Option<String>,
  pub turn_clock: Option<i64>,
  pub tags_co_id: Option<i64>,
  pub tags_co_power: Option<i64>,
  pub tags_co_max_power: Option<i64>,
  pub tags_co_max_spower: Option<i64>,
  pub interface: Option<String>,
}

pub fn read_players(input: &str) -> IResult<&str, Vec<Player>> {
  // s:7:"players";
  let (input, _) = tag("a:")(input)?;
  let (input, length) = read_next_length(input)?;
  let (input, _) = tag("{")(input)?;

  let mut players = Vec::new();
  let mut input_loop = input;

  for _i in 0..length {
    let input = input_loop;
    let (input, _player_index) = read_i64(input)?;
    let (input, _) = tag(r#"O:10:"awbwPlayer":"#)(input)?;
    let (input, num_fields) = read_next_length(input)?;
    let (input, _) = tag("{")(input)?;
    let mut player_builder = PlayerBuilder::default();

    input_loop = input;
    for _j in 0..num_fields {
      let input = input_loop;
      let (input, entry) = read_string(input)?;
      let input = match entry {
        "id" => {
          let (input, value) = read_i64(input)?;
          player_builder.id(value);
          input
        }
        "users_id" => {
          let (input, value) = read_i64(input)?;
          player_builder.user_id(value);
          input
        }
        "games_id" => {
          let (input, value) = read_i64(input)?;
          player_builder.games_id(value);
          input
        }
        "countries_id" => {
          let (input, value) = read_i64(input)?;
          player_builder.countries_id(value);
          input
        }
        "co_id" => {
          let (input, value) = read_i64(input)?;
          player_builder.co_id(value);
          input
        }
        "funds" => {
          let (input, value) = read_i64(input)?;
          player_builder.funds(value);
          input
        }
        "turn" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.turn(value.map(|v| v.to_owned()));
          input
        }
        "email" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.email(value.map(|v| v.to_owned()));
          input
        }
        "uniq_id" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.uniq_id(value.map(|v| v.to_owned()));
          input
        }
        "eliminated" => {
          let (input, value) = read_bool(input)?;
          player_builder.eliminated(value);
          input
        }
        "last_read" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.last_read(value.map(|v| v.to_owned()));
          input
        }
        "last_read_broadcasts" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.last_read_broadcasts(value.map(|v| v.to_owned()));
          input
        }
        "emailpress" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.emailpress(value.map(|v| v.to_owned()));
          input
        }
        "signature" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.signature(value.map(|v| v.to_owned()));
          input
        }
        "co_power" => {
          let (input, value) = read_i64(input)?;
          player_builder.co_power(value);
          input
        }
        "co_power_on" => {
          let (input, value) = read_bool(input)?;
          player_builder.co_power_on(value);
          input
        }
        "order" => {
          let (input, value) = read_i64(input)?;
          player_builder.order(value);
          input
        }
        "accept_draw" => {
          let (input, value) = read_bool(input)?;
          player_builder.accept_draw(value);
          input
        }
        "co_max_power" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.co_max_power(value);
          input
        }
        "co_max_spower" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.co_max_spower(value);
          input
        }
        "co_image" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.co_image(value.map(|v| v.to_owned()));
          input
        }
        "team" => {
          let (input, value) = read_string(input)?;
          player_builder.team(value.to_owned());
          input
        }
        "aet_count" => {
          let (input, value) = read_i64(input)?;
          player_builder.aet_count(value);
          input
        }
        "turn_start" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.turn_start(value.map(|v| v.to_owned()));
          input
        }
        "turn_clock" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.turn_clock(value);
          input
        }
        "tags_co_id" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.tags_co_id(value);
          input
        }
        "tags_co_power" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.tags_co_power(value);
          input
        }
        "tags_co_max_power" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.tags_co_max_power(value);
          input
        }
        "tags_co_max_spower" => {
          let (input, value) = read_i64_opt(input)?;
          player_builder.tags_co_max_spower(value);
          input
        }
        "interface" => {
          let (input, value) = read_string_opt(input)?;
          player_builder.interface(value.map(|v| v.to_owned()));
          input
        }
        unknown => panic!("Replay player data contained unknown entry: {unknown}"),
      };
      input_loop = input;
    }
    players.push(player_builder.build().unwrap());

    let input = input_loop;
    let (input, _) = tag("}")(input)?;
    input_loop = input;
  }

  let input = input_loop;

  let (input, _) = tag("}")(input)?;
  Ok((input, players))
}
