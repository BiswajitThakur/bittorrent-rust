use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next().unwrap() {
        '0'..='9' => encoded_value
            .split_once(':')
            .and_then(|(left, right)| {
                let len = left.parse::<usize>().ok()?;
                Some((
                    serde_json::Value::String(String::from(&right[0..len])),
                    &right[len..],
                ))
            })
            .unwrap(),
        'i' => encoded_value
            .split_at(1)
            .1
            .split_once('e')
            .and_then(|(left, right)| {
                let val = left.parse::<i64>().ok()?;
                Some((serde_json::Value::Number(val.into()), right))
            })
            .unwrap(),
        'l' => {
            let mut arr = Vec::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (v, rem) = decode_bencoded_value(rest);
                arr.push(v);
                rest = rem;
            }
            return (arr.into(), &rest[1..]);
        }
        'd' => {
            let mut map = serde_json::Map::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (k, rem) = decode_bencoded_value(rest);
                let key = match k {
                    serde_json::Value::String(k) => k,
                    _ => panic!("key must be string..."),
                };
                let (val, rem) = decode_bencoded_value(rem);
                map.insert(key, val);
                rest = rem;
            }
            return (map.into(), &rest[1..]);
        }
        _ => {
            unreachable!()
        }
    }
}

#[test]
fn test_bencode() {
    let input = "5:hello";
    assert_eq!(
        (serde_json::Value::String("hello".into()), ""),
        decode_bencoded_value(input)
    );
    let input = "6:hellohiii";
    assert_eq!(
        (serde_json::Value::String("helloh".into()), "iii"),
        decode_bencoded_value(input)
    );
    let input = "5:hello5:hello";
    assert_eq!(
        (serde_json::Value::String("hello".into()), "5:hello"),
        decode_bencoded_value(input)
    );
    let input = "i52e";
    assert_eq!(
        (serde_json::Value::Number(52.into()), ""),
        decode_bencoded_value(input)
    );
    let input = "i-52e";
    assert_eq!(
        (serde_json::Value::Number((-52).into()), ""),
        decode_bencoded_value(input)
    );
    let input = "i-52eeabc";
    assert_eq!(
        (serde_json::Value::Number((-52).into()), "eabc"),
        decode_bencoded_value(input)
    );
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    pub p_len: usize,
    pub pieces: ByteBuf,
    #[serde(flatten)]
    pub keys: Keys,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keys {
    SingleFile { length: usize },
    MultiFile { files: Vec<File> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub length: u64,
    pub path: Vec<String>,
}

pub fn parse_torrent_file<P: AsRef<Path>>(path: P) -> Torrent {
    let b = std::fs::read(path).unwrap();
    serde_bencode::de::from_bytes(&b).unwrap()
}
