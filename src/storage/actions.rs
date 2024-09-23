use std::{path::Path, sync::Arc};

use parquet::{
  data_type::ByteArray,
  file::{properties::WriterProperties, writer::SerializedFileWriter},
  schema::parser::parse_message_type,
};

use crate::parser::action::TurnActions;

pub fn save_to_file(path: &Path, mut data: Vec<TurnActions>) {
  let message_type = "
    message schema {
      REQUIRED INT64 game_id;
      REQUIRED INT64 player_id;
      REQUIRED INT64 day;
      REQUIRED BYTE_ARRAY actions;
    }
  ";

  let schema = Arc::new(parse_message_type(message_type).unwrap());
  let props = WriterProperties::builder()
    .set_column_encoding(
      "game_id".into(),
      parquet::basic::Encoding::DELTA_BINARY_PACKED,
    )
    .set_column_encoding(
      "player_id".into(),
      parquet::basic::Encoding::DELTA_BINARY_PACKED,
    )
    .set_column_encoding(
      "player_day".into(),
      parquet::basic::Encoding::DELTA_BINARY_PACKED,
    )
    .set_column_compression(
      "actions".into(),
      parquet::basic::Compression::ZSTD(parquet::basic::ZstdLevel::try_new(22).unwrap()),
    )
    .build();
  let file = std::fs::File::create(&path).unwrap();
  let mut writer = SerializedFileWriter::new(file, schema, Arc::new(props)).unwrap();

  let mut rows = 0usize;

  data.sort_by(|l, r| l.player_id.partial_cmp(&r.player_id).unwrap());

  {
    let mut row_group_writer = writer.next_row_group().unwrap();
    // GAME_ID
    {
      let values: Vec<i64> = data.iter().map(|_| 1246603).collect();
      if let Some(mut writer) = row_group_writer.next_column().unwrap() {
        rows += writer
          .typed::<parquet::data_type::Int64Type>()
          .write_batch(&values, None, None)
          .unwrap();
        let _ = writer.close();
      }
    }
    // PLAYER_ID
    {
      let values: Vec<i64> = data.iter().map(|v| v.player_id).collect();
      if let Some(mut writer) = row_group_writer.next_column().unwrap() {
        rows += writer
          .typed::<parquet::data_type::Int64Type>()
          .write_batch(&values, None, None)
          .unwrap();
        let _ = writer.close();
      }
    }
    // DAY
    {
      let values: Vec<i64> = data.iter().map(|v| v.day).collect();
      if let Some(mut writer) = row_group_writer.next_column().unwrap() {
        rows += writer
          .typed::<parquet::data_type::Int64Type>()
          .write_batch(&values, None, None)
          .unwrap();
        let _ = writer.close();
      }
    }
    // ACTION JSON
    {
      let values: Vec<ByteArray> = data
        .into_iter()
        .map(|v| {
          ByteArray::from(
            serde_json::to_string(&serde_json::Value::Array(v.actions))
              .unwrap()
              .into_bytes(),
          )
        })
        .collect();
      if let Some(mut writer) = row_group_writer.next_column().unwrap() {
        rows += writer
          .typed::<parquet::data_type::ByteArrayType>()
          .write_batch(&values, None, None)
          .unwrap();
        let _ = writer.close();
      }
    }
    let _ = row_group_writer.close();
  }
  writer.close().unwrap();

  println!("Wrote {}", rows);

  // let bytes = std::fs::read(&path).unwrap();
}

#[cfg(test)]
mod test {
  use std::{
    fs::File,
    io::{self, BufRead},
  };

  use crate::parser::action::read_turn_actions;

  use super::*;

  fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where
    P: AsRef<Path>,
  {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
  }

  #[test]
  fn test() {
    let path = Path::new("./sample.parquet");

    let lines = read_lines("../a1246603.uncompressed").unwrap();
    let mut turns = Vec::new();

    for line in lines.flatten() {
      let (_input, turn) = read_turn_actions(&line).unwrap();
      turns.push(turn);
    }
    save_to_file(&path, turns);
  }
}
