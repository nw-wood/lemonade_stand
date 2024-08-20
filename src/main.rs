use std::{io, thread};
use std::time::Duration;
use rand::{Rng, thread_rng};

#[derive(Debug)]
struct LemonadeStand {
    money:  usize,
    lemons: usize, //4
    sugar:  usize, //1
    water:  usize, //2
    ice:    usize, //2
    cups:   usize, //8
   pitcher: usize,
}

const PITCHER_LEMONS: usize = 4;
const PITCHER_WATER: usize = 2;
const PITCHER_SUGAR: usize = 4;
const PITCHER_CUPS: usize = 8;
const PITCHER_ICE: usize = 2;
const PITCHER_SIZE: usize = PITCHER_CUPS - 1;

const START_COUNT: usize = 30;

const EARNINGS_PER_CUP: usize = 1;

impl LemonadeStand {

    fn new() -> Self {
        Self {
            money: START_COUNT,
            lemons: START_COUNT,
            sugar: START_COUNT,
            water: START_COUNT,
            ice: START_COUNT,
            cups: START_COUNT,
            pitcher: 0
        }
    }

    fn run_day(&mut self) {

        println!("\n🍋 fresh lemonade for sale!\n");
        hesitate();

        let mut missed_sales: usize = 0;
        let mut earned_sales: usize = 0;
        for i in 0..13 {
            let mut random = thread_rng();
            let random = random.gen_range(0..3);

            if 8 + i <= 12 { println!("time of day: {}{} {}\n", i + 8, ":00 am".to_string(), "$".repeat(random)); }
            else { println!("time of day: {}{} {}\n", i - 4, ":00 pm".to_string(), "$".repeat(random)); }

            for _ in 0..random {
                if self.pitcher > 0 {
                    self.pitcher -= 1;
                    self.money += 1;
                    earned_sales += 1;
                } else {
                    if  self.lemons >= PITCHER_LEMONS &&
                        self.sugar >= PITCHER_SUGAR &&
                        self.water >= PITCHER_WATER &&
                        self.ice >= PITCHER_ICE &&
                        self.cups >= PITCHER_CUPS { //🧠

                        self.lemons -= PITCHER_LEMONS;
                        self.sugar -= PITCHER_SUGAR;
                        self.water -= PITCHER_WATER;
                        self.ice -= PITCHER_ICE;
                        self.cups -= PITCHER_CUPS;
                        self.pitcher += PITCHER_SIZE;
                        self.money += EARNINGS_PER_CUP;
                        earned_sales += 1;
                    }
                    else { missed_sales += 1; }
                }
            }
            if missed_sales > 0 { println!("🚫 lost {missed_sales} x 💵 because supplies dwindled!\n")}
            hesitate();
        }
        println!("sold {earned_sales} cups! 🌚\n");
        hesitate();
        if self.lemons <= PITCHER_LEMONS * 2 || self.sugar <= PITCHER_SUGAR * 2 || self.water <= PITCHER_WATER * 2 || self.ice <= PITCHER_ICE * 2 || self.cups <= PITCHER_CUPS * 2 {
            println!("⚠️ getting low on supplies!\n");
        }
        hesitate();
    }

    fn shop(&mut self, command: &str, quantity: usize) {
        let mut cost: usize = 0;
        let mut item: & mut usize = &mut 0;
        match command {
            "lemons" => { cost = 1; item = &mut self.lemons}
            "ice" =>    { cost = 1; item = &mut self.ice}
            "water" =>  { cost = 1; item = &mut self.water}
            "sugar" =>  { cost = 1; item = &mut self.sugar}
            "cups" =>   { cost = 1; item = &mut self.cups}
            _ => {}
        }
        if cost * quantity <= self.money {
            self.money -= cost * quantity;
            *item += quantity;
            println!("\n💸 purchased {quantity} x {command} for ${}\n", cost * quantity);
            hesitate();
        } else {
            println!("\n💵🫰 you need more money to buy that\n");
        }
    }
}

fn main() {

    let mut stand = LemonadeStand::new();

    'game_loop: loop {
        println!("🍋🧊\n\n{stand:?}\n");
        'input_loop: loop {
            hesitate();
            println!("what do you want to do? (buy 3 lemons, start, quit)\n");

            let mut input= String::new(); //take users input from term
            if let Ok(_) = io::stdin().read_line(&mut input) {}
            else { println!("invalid input\n"); return; }

            let mut input = input.split_whitespace(); //user input -> input into iter of args
            let mut command = input.next().unwrap_or("");

            match command { //consume iter by matching args in order (buy, 10, lemons)
                "buy" => {
                    let number = input.next().unwrap_or("0");
                    if let Ok(quantity) = number.parse::<usize>() {
                        command = input.next().unwrap_or("invalid");
                        match command {
                            "lemons"|"ice"|"water"|"cups"|"sugar" => stand.shop(command, quantity),
                            _ => { println!("buy lemons, ice, water, sugar, or cups; followed by a number\n") }
                        }
                    }
                }
                "start" =>  { break 'input_loop; }
                "quit" =>   {
                    println!("\n👋🤠\n");
                    break 'game_loop; }
                _ => {
                    println!("\n✖️\n");
                }
            }
        }
        stand.run_day(); //progress game
    }
}

fn hesitate() {
    thread::sleep(Duration::from_millis(500));
}