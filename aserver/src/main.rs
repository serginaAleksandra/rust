extern crate rand;
use std::thread;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time::Duration;
use std::io;
use std::str::from_utf8;

const MAX_CLIENTS: usize = 10;
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

fn handle_client(mut stream: TcpStream) {
    let mut buff = vec![0; MSG_SIZE];
    let mut key = [0 as u8; 10];
    let mut message = [0 as u8;50];

    match stream.read_exact(&mut buff) {
        Ok(_) => {
            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            let msg = String::from_utf8(msg).expect("Invalid utf8 message");

            println!("{:?}", msg);

            stream.read(&mut key);
            stream.read(&mut message);

            let received_hash = from_utf8(&buff).unwrap();
            let received_key = from_utf8(&key).unwrap();
            let new_key = next_session_key(&received_hash,&received_key);
            let result = new_key.clone().into_bytes();

            stream.write(&result).unwrap();
            send_message(&stream);


        },
        Err(_) => {
            println!("closing connection with::");
        }
    }                    
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3343").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port");

    let mut clientsConnected = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                clientsConnected = clientsConnected + 1;

                if clientsConnected <= MAX_CLIENTS {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    thread::spawn(move||  {
                    // connection succeeded
                        handle_client(stream)
                        
                    });
                } else {
                    println!("Reached connections limit: {}.", MAX_CLIENTS);

                    break;
                }

                
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}


fn get_session_key() -> String 
{
    let mut key = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10 
    {
        let num = rng.gen_range(1..10);
        let ch = char::from_digit(num, 10).unwrap();
        key.push(ch);
    }

    return key;
}

fn get_hash_str() -> String 
{
    let mut hash_str = String::new();
    let mut rng = rand::thread_rng();

    for _i in 0..5 
    {
        let num = rng.gen_range(1..7);
        let ch = char::from_digit(num, 10).unwrap();
        hash_str.push(ch);
    }

    return hash_str;
}

fn next_session_key(hash_str: &str, session_key: &str) -> String 
{
    if hash_str.is_empty() 
    {
        return "HASH CODE IS EMPTY".to_string()
    }

    for ch in hash_str.chars() 
    {
        if !ch.is_ascii_digit() 
        {
            return "HASH CODE CONTAINS NON-DIGIT LETTER".to_string()
        }
    }

    let mut result = 0;

    for ch in hash_str.chars() 
    {
        let l = ch.to_string();
        result += calc_hash(session_key.to_string(), l.parse::<u64>().unwrap()).parse::<u64>().unwrap();
    }

    return result.to_string();
}

fn calc_hash(key: String, value: u64) -> String 
{
    match value
    {
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
    
            for _i in 0..key.len() 
            {
                ch = ((key.chars().nth(_i).unwrap() as u8) ^ 43) as char;
                if !ch.is_ascii_digit() 
                {
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
