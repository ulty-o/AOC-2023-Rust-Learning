
use std::str;
use std::fs;
fn main() {
    
    const IS_PART_ONE:bool = true;

    let file_path = "./sample.input";
    println!("File path {file_path}");

    let input = fs::read_to_string(file_path).expect("Should have been able to read file"); 
    //println!("With text:\n{input}");

    if IS_PART_ONE {
        let lines: Vec<&str> = input.trim().split('\n').collect();
        let mut total_sum =0;
        for line in lines {
            //println!("the value is: {line}");
            //get first and last digit
            let mut first_digit: char = '\0'; //\0 is the null char
            let mut last_digit: char = ' ';
            for c in line.chars(){
                if c.is_numeric(){
                    if first_digit == '\0' {
                        first_digit = c;
                        last_digit = c;
                    }else{
                        last_digit = c;
                    }
                } 
            }
            let mut result = String::new(); 
            result.push_str(&first_digit.to_string());
            result.push_str(&last_digit.to_string());
            println!("Result inner: {result}");
            if !result.trim().is_empty() {
                let parsed_result:i32 = result.trim().parse().unwrap_or(0); //trim and parse string to i32
                total_sum += parsed_result;
            } 
       
        }
        println!("Result: {total_sum}"); 
    }else{
        let lines: Vec<&str> = input.trim().split('\n').collect(); //fill vector with line, \n delimiter
        let mut total_sum = 0; 
        for line in lines{ 
            let mut digits:Vec<i32> = Vec::new();  
            tokenize_word_numbers(line,&mut digits);
            
            let joined_digits = digits.first().unwrap_or(&0)*10 + digits.last().unwrap_or(&0);
            //println!("Line: {joined_digits}");
            total_sum += joined_digits;
        }
        println!("Total: {total_sum}");
    }
}

fn tokenize_word_numbers(input: &str, output: &mut Vec<i32>){

    let input = input.to_lowercase();
    for (i,el) in input.chars().enumerate(){
        let c = el;
        match c {
            'o' => { if input[i..].starts_with("one") {output.push(1)}; },
            'n' => { if input[i..].starts_with("nine") {output.push(9)}; },
            'e' => { if input[i..].starts_with("eight") {output.push(8)}; },
            's' => { 
                if input[i..].starts_with("seven") {
                    output.push(7);
                } else if input[i..].starts_with("six") {
                    output.push(6); 
                } 
            },
            'f' => { 
                if input[i..].starts_with("five") {
                    output.push(5);
                } else if input[i..].starts_with("four") {
                    output.push(4); 
                } 
            },
            't' => { 
                if input[i..].starts_with("two") {
                    output.push(2);
                } else if input[i..].starts_with("three") {
                    output.push(3); 
                } 
            },
            '0'..='9' => if let Some(digit) = c.to_digit(10) { output.push(digit as i32); },
            _ => (),
        };
    }
}

