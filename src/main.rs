use itertools::Itertools;
use std::{io::{stdin, stdout}};
use std::process;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};
 
#[derive(Debug)]
struct KnuthMagic {
   guess: Vec<char>,
   value: usize,
   score: (usize, usize)
}
 
impl KnuthMagic {
   fn new(guess: Vec<char>, score: (usize, usize)) -> Self {
       Self {
       guess: guess,
       value: 1,
       score: score,
       }
   }
 
   fn add_point(&mut self) {
       self.value += 1;
   }
}
 
fn main() {
   clean_buffer();
   print_service("init");
   let mut t = generate_digits();
 
   loop {
       println!("add_guess");
       let mut value = String::new();
       match stdin().read_line(&mut value) {
           Ok(_) => {
               println!("You have guessed {}", value.trim());
           }
           Err(error) => println!("error: {}", error),
       }
       let value_vec: Vec<char> = value.trim().chars().collect();
 
       let mut guess_bulls = String::new();
       println!("How many bulls?");
       match stdin().read_line(&mut guess_bulls) {
           Ok(_) => { println!("bulls: {}", guess_bulls)}
           Err(error) => println!("error: {}", error),
       }
 
       let mut guess_cows = String::new();
       println!("How many cows?");
       match stdin().read_line(&mut guess_cows) {
           Ok(_) => { println!("bulls: {}", guess_cows)}
           Err(error) => println!("error: {}", error),
       }
 
       let guessed_score = (guess_bulls.trim().parse::<usize>().unwrap(), guess_cows.trim().parse::<usize>().unwrap());
 
       println!("{:?}",guessed_score);
 
       t.retain( |x| calculate_score(&x, &value_vec) == guessed_score);
 
       next_guess(&t);
       println!("{}", t.len());
   }
}
 
fn calculate_score(given_digits: &Vec<char>, guessed_digits: &Vec<char>) -> (usize, usize) {
   let mut bulls = 0;
   let mut cows = 0;
   for i in 0..6 {
       let pos: Option<usize> = guessed_digits.iter().position(|&a| -> bool {a == given_digits[i]});
       match pos {
           None              => (),
           Some(p) if p == i => bulls += 1,
           Some(_)           => cows += 1
       }
   }
   return (bulls, cows);
}
 
fn next_guess(guesses: &Vec<Vec<char>>) {
   let mut chosen_knuths = vec![];
   for g in guesses.iter() {
       let mut knuth_collection:Vec<KnuthMagic> = vec![];
       for c in guesses.iter() {
           let k = KnuthMagic::new(g.to_vec(), calculate_score(&g, &c));
           if known_knuth(&knuth_collection, &k) {
               find_knuth(&mut knuth_collection, &k)
           } else {
               knuth_collection.push(k)
           }
       }
       chosen_knuths.push(knuth_collection.into_iter().max_by_key(|x| x.value).unwrap());
   }
   let best_guess = chosen_knuths.into_iter().max_by_key(|x| x.value).unwrap();
   println!("{:?}", best_guess);
}
 
fn find_knuth(cc: &mut Vec<KnuthMagic>, knuth: &KnuthMagic) {
   let knuth_match = cc.iter_mut().find(|x| x.guess == knuth.guess && x.score == knuth.score).unwrap();
   knuth_match.add_point();
}
 
fn known_knuth(cc: &Vec<KnuthMagic>, knuth: &KnuthMagic) -> bool {
   cc.iter().find(|x| x.guess == knuth.guess && x.score == knuth.score).is_some()
}
 
fn generate_digits() -> Vec<Vec<char>> {
   vec!('0','1', '2', '3', '4', '5', '6', '7', '8', '9').into_iter().permutations(6).collect_vec()
}
 
 
fn print_service(request: &str) {
   match request {
       "init" => {
           println!("");
           println!("");
           println!("Welcome to the mastermind solver");
           println!("--------------------------------");
           println!("");
           println!("");
       }
       "add_guess" => {
           println!("");
           println!("");
           println!("Add your new guess:");
           println!("");
           println!("");
       }
       _ => {
           println!("Unknown request");
       }
   }
}
 
fn clean_buffer() {
   let mut stdout = stdout();
   execute!(stdout, Clear(ClearType::All)).unwrap();
   execute!(stdout, cursor::MoveTo(0,0));
}
