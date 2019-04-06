use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let total = iterator.next().unwrap().unwrap();
    
    for line in 0..total.parse::<u64>().unwrap() {
        let _size  = iterator.next().unwrap().unwrap().parse::<u64>().unwrap();
        let map = iterator.next().unwrap().unwrap();

        let mut out:String = "".to_string();

        for ch in map.chars(){
            if ch == 'S'{
                out += "E";
            }else{
                out += "S";
            }
        }

        println!("Case #{}: {}", line+1, out)
        
    }
}
