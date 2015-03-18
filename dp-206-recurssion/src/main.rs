use std::io;

fn main() {
    let inputs = get_input();

    recur(get_operators(&inputs), &inputs);
}


//Function to get inputs from the command line.
//Returns Vec<String>
fn get_input() -> Vec<String> {
    let mut reader = io::stdin();

    let mut operators = String::new();
    let mut start_value = String::new();
    let mut terms = String::new();

    println!("Please enter your operators.");
    reader
        .read_line(&mut operators)
        .ok()
        .expect("whoops");

    println!("What is the starting value?");
    reader
        .read_line(&mut start_value)
        .ok()
        .expect("whoops");

    println!("How many terms should be calculated?");
    reader
        .read_line(&mut terms)
        .ok()
        .expect("whoops");

    operators.pop();
    start_value.pop();
    terms.pop();

    vec![operators, start_value, terms]
}

//Takes the operators that the user passed in, and parses them
//Returns a Vector of a tuple pair of characters, Vec<(char, char)>
fn get_operators(inputs: &Vec<String>) -> Vec<(char, i32)> {
    let mut pairs = Vec::new();
    let i: &str = &inputs[0];

    // Take the string, and parse it as an array or "words" by cutting at whitespace
    let iteration_pairs = i.words();

    // Loop through each word, and assign each character to the tuple.
    for pair in iteration_pairs {
        let mut c = pair.chars();
        pairs.push(
            (
                c.next().unwrap(),
                char::to_digit(c.next().unwrap(), 36).unwrap() as i32
            )
        );
    }
    pairs
}

//Handles the actual recursion.  Prints each case to the command line
//Returns: nothing
fn recur(operators: Vec<(char, i32)>, inputs: &Vec<String>) {
    let mut value: i32 = inputs[1].parse().ok().expect("not an int");
    let terms: u32 = inputs[2].parse().ok().expect("not an int");


     // Take the operator string -> vector splitting at <space>
     // turn each value in the vector into a tuple
     // use a match to get opperator, and pass value

    // Start a loop for each term
    let mut n = 0;
    while n <= terms {
        for i in operators {
            println!("{:?}", i);
        }
        // match(i.next()) {
        //     ('+', x) => value += x,
        //     ('-', x) => value -= x,
        //     ('*', x) => value *= x,
        //     ('/', x) => value /= x,
        //     (_, _)   => panic!("What did you give me?!"),
        // }
        // println!("Term {:?}: {:?}", n, value);
        n += 1;
    }
}
























