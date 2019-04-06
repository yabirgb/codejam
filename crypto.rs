use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;
use std::collections::HashMap;

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let total = iterator.next().unwrap().unwrap();
    
    for line in 0..total.parse::<u64>().unwrap() {
        let alphabet:Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

        // Read the data
        let data:Vec<u64> = iterator.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let numbers:Vec<u64> = iterator.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();


        //Store primes found
        let mut primes = BTreeSet::new();
        let mut letters_code = Vec::new();
        
        for i in 0..numbers.len()-1{
            let gcdvalue = gcd(numbers[i], numbers[i+1]);
            let first = numbers[i]/gcdvalue;
            let second = numbers[i+1]/gcdvalue;
            
            primes.insert(gcdvalue);
            primes.insert(first);
            primes.insert(second);
            
            letters_code.push(first);
            letters_code.push(gcdvalue);

            if data[1] % 2 == 1 && i == numbers.len()-2{
                letters_code.push(second);
            }
            
        }

        let mut dic:HashMap<u64, char> = HashMap::new();


        // EXtract our word
        let mut finals = Vec::new();
        finals.push(letters_code[0]);
        for(index, number) in letters_code.iter().skip(1).enumerate(){
            if index%2 == 0{
                finals.push(*number);
            }
        }

        if data[1] % 2 == 1 && letters_code.len() > 2{
            finals.push(letters_code[letters_code.len()-1]);
        }

        // end

        //Load primes in dic
        let p:Vec<(&u64, &char)> = primes
            .iter()
            .zip(alphabet.iter()).collect();

        for pair in p{
            dic.insert(*pair.0, *pair.1);
        }
        //end

        // cook output
        let mut result:String = "".to_string();

        for number in finals{
            result += &dic.get(&number).unwrap().to_string();
        }
        // end
            
        println!("Case #{}: {}", line+1, result);
        
    }

}
