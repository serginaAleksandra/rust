extern crate rand;
use std::thread;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::time::Duration;
use std::io;

const MSG_SIZE: usize = 32;

fn send_message(mut stream: &TcpStream) {
    println!("Your message: ");

            let mut message = String::new();

            io::stdin().read_line(&mut message);

            let mut buff = message.clone().into_bytes();
            buff.resize(MSG_SIZE, 0);
            stream.write(&buff).expect("writing to socket failed");
            println!("message sent {:?}", message);
}

fn main() {

    let mut client = TcpStream::connect("localhost:3343");

    match client {
        
        Ok(mut stream) => {

            println!("Successfully connected to server");

            let mut data = [0 as u8; 50];
            let mut rep = [0 as u8; 50];

            loop {

                let hash_str = get_hash_str();
                let session_key = get_session_key();

                let next_key = next_session_key(&hash_str, &session_key);

                send_message(&stream);

                stream.write(&hash_str.into_bytes()).unwrap();
                stream.write(&session_key.into_bytes()).unwrap();

                let mut buff = vec![0; MSG_SIZE];

                match stream.read_exact(&mut buff) {
                    Ok(size) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        stream.read(&mut rep);
                        let received_key = from_utf8(&buff[0..size]).unwrap();

                        if received_key == next_key {
                            println!("CLIENT KEY: {}\nSERVER KEY: {}", next_key, received_key);
                        } else { break; }

                        println!("{:?}", msg);

                    },
                Err(_) => {
                    println!("closing connection with::");
                }
            }     
        }
    }

    Err(e) => {println!("CONNECTION FAILED: {}", e);}
    }

}


fn get_session_key() -> String {

    let mut key = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10 {

        let num = rng.gen_range(1..10);
        let ch = char::from_digit(num, 10).unwrap();
        key.push(ch);
    }

    return key;
}

fn get_hash_str() -> String {

    let mut hash_str = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..5 {

        let num = rng.gen_range(1..7);
        let ch = char::from_digit(num, 10).unwrap();
        hash_str.push(ch);
    }

    return hash_str;
}

fn next_session_key(hash_str: &str, session_key: &str) -> String {

    if hash_str.is_empty() {
        return "HASH CODE IS EMPTY".to_string()
    }

    for ch in hash_str.chars() {
        if !ch.is_ascii_digit() {
            return "HASH CODE CONTAINS NON-DIGIT LETTER".to_string()
        }
    }

    let mut result = 0;

    for ch in hash_str.chars() {
        let l = ch.to_string();
        result += calc_hash(session_key.to_string(), l.parse::<u64>().unwrap()).parse::<u64>().unwrap();
    }

    return result.to_string();
}

fn calc_hash(key: String, value: u64) -> String {
    match value {
        1=>{
            let chp = "00".to_string() + &(key[0..5].parse::<u64>().unwrap() % 97).to_string();
            return chp[chp.len() - 2..chp.len()].to_string()
            }

        2=>{
            let reverse_key = key.chars().rev().collect::<String>();
            return reverse_key + &key.chars().nth(0).unwrap().to_string()
            }

        3=>{
            return key[key.len() - 5..key.len()].to_string() + &key[0..5].to_string()
            }

        4=>{
            let mut num = 0;
            for _i in 1..9 
            {
                num += key.chars().nth(_i).unwrap().to_digit(10).unwrap() as u64 + 41;
            }
            return num.to_string()
            }

        5=>{
            let mut ch: char;
            let mut num = 0;
    
            for _i in 0..key.len() {
                ch = ((key.chars().nth(_i).unwrap() as u8) ^ 43) as char;
                if !ch.is_ascii_digit() {
                    ch = (ch as u8) as char;
                }
                num += ch as u64;
            }
            return num.to_string()
            }

        _=>{
            return (key.parse::<u64>().unwrap() + value).to_string()
            }
    }
}
