use std::io;
use std::io::prelude::*;

fn decode(number:String)->(String, String){
    let mut st1 = "".to_string();
    let mut st2 = "".to_string();

    for s in number.chars(){
        let ch = s.to_string();
        if ch == "4"{
            st1 += "3";
            st2 += "1";
        }else{
            st1 += &ch;
            st2 += "0";
        }
    }

    return (st1, st2)
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let total = iterator.next().unwrap().unwrap();
    
    for line in 0..total.parse::<u64>().unwrap() {
        let number = iterator.next().unwrap().unwrap();
        let result = decode(number);
        println!("Case #{}: {} {}", line+1, result.0,result.1);
        
    }

}
