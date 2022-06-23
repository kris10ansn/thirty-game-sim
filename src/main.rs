use rand::Rng;

fn roll_die() -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=6);
}

trait Strategy {
    fn pick_dice(&self, dice: &Vec<u8>) -> Vec<u8>;
}

struct RandomStrategy {}
impl Strategy for RandomStrategy {
    fn pick_dice(&self, dice: &Vec<u8>) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        return vec![dice[rng.gen_range(0..dice.len())]];
    }
}

struct OnlySixesStrategy {}
impl Strategy for OnlySixesStrategy {
    fn pick_dice(&self, dice: &Vec<u8>) -> Vec<u8> {
        let mut picks: Vec<u8> = Vec::new();

        dice.iter().for_each(|dice_value| {
            if *dice_value == 6 {
                picks.push(6)
            }
        });

        if picks.len() == 0 {
            let max = dice.iter().max().unwrap();
            picks.push(*max);
        }

        picks
    }
}

fn play_game(strategy: &dyn Strategy) -> u8 {
    let mut dice: Vec<u8> = Vec::new();
    let mut score: u8 = 0;

    fn roll(dice: &mut Vec<u8>, n: u8) {
        if dice.len() > 0 {
            dice.clear();
        }

        for _ in 1..=n {
            dice.push(roll_die());
        }
    }

    roll(&mut dice, 6);

    while dice.len() > 0 {
        let picked = strategy.pick_dice(&dice);

        for die_value in &picked {
            score += die_value;

            let index = dice.iter().position(|&value| value == *die_value).unwrap();

            dice.swap_remove(index);
        }

        let n = dice.len() as u8;
        roll(&mut dice, n);
    }

    score
}

fn test_strategy(strategy: &dyn Strategy, iterations: u32) -> f32 {
    let mut total: u32 = 0;

    for _ in 0..iterations {
        total += play_game(strategy) as u32;
    }

    total as f32 / iterations as f32
}

fn main() {
    let iterations: u32 = 10_000;

    let random_strategy = RandomStrategy {};
    let only_sixes_strategy = OnlySixesStrategy {};

    println!(
        "Average score - Random strategy - {} iterations: {}",
        iterations,
        test_strategy(&random_strategy, iterations)
    );

    println!(
        "Average score - Only sixes strategy - {} iterations: {}",
        iterations,
        test_strategy(&only_sixes_strategy, iterations)
    );
}
