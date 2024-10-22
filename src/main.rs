#![allow(unused)]
use std::{env, iter::Peekable, ops::Range, str::FromStr};

use bittorrent_starter_rust::decode_bencoded_value;

fn get_ben_str<I: Iterator<Item = (usize, char)>>(iter: &mut Peekable<I>) -> Option<Range<usize>> {
    match iter.peek() {
        Some(v) if v.1.is_ascii_digit() => {
            let mut len: usize = 0;
            let mut i = 0;
            let mut has_colon = false;
            for (index, c) in iter.by_ref() {
                match c {
                    ':' => {
                        i = index;
                        has_colon = true;
                        break;
                    }
                    v => {
                        len = (len * 10) + (v as usize - '0' as usize);
                    }
                }
            }
            if has_colon {
                i += 1;
                for _ in 0..len {
                    iter.next();
                }
                Some(i..i + len)
            } else {
                None
            }
        }
        _ => None,
    }
}

// i52e => 52, i-52e => -52
fn get_ben_num<I: Iterator<Item = (usize, char)>>(iter: &mut Peekable<I>) -> Option<Range<usize>> {
    let mut start = 0;
    let mut end = 0;
    let mut has_e = false;
    if let Some((index, c)) = iter.peek() {
        if *c == 'i' {
            start = *index;
            iter.next();
        } else {
            return None;
        }
    }
    for (index, c) in iter.by_ref() {
        if c == 'e' {
            end = index;
            has_e = true;
            break;
        }
    }
    start += 1;
    if has_e {
        Some(start..end)
    } else {
        None
    }
}

//let mut iter = encoded_value.chars().enumerate().peekable();
/*
let first_char = iter.peek().unwrap();
match first_char.1 {
    '0'..='9' => {
        if let Some(range) = get_ben_str(iter) {
            println!("{:?}", iter.peek());
            return serde_json::Value::String(String::from(&encoded_value[range]));
        }
    }
    'i' => {
        if let Some(range) = get_ben_num(iter) {
            println!("{:?}", iter.peek());
            return serde_json::Value::Number(
                serde_json::Number::from_str(&encoded_value[range]).unwrap(),
            );
        }
    }
    'l' => {
        let mut arr = Vec::new();
        iter.next();
        loop {
            if let Some(range) = get_ben_num(iter) {
                arr.push(serde_json::Value::Number(
                    serde_json::Number::from_str(&encoded_value[range]).unwrap(),
                ));
            } else if let Some(range) = get_ben_str(iter) {
                arr.push(serde_json::Value::String(String::from(
                    &encoded_value[range],
                )));
            } else {
                if let Some((_, c)) = iter.peek() {
                    if *c == 'l' {
                        arr.push(decode_bencoded_value(encoded_value, iter));
                    } else {
                        iter.next();
                    }
                }
            }
            if iter.peek().is_none() {
                break;
            }
        }
        return serde_json::Value::Array(arr);
    }
    _ => {}
}*/
// panic!("Unhandled encoded value: ")

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let mut iter = encoded_value.as_str().chars().enumerate().peekable();
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.0);
    } else {
        println!("unknown command: {}", args[1])
    }
}
