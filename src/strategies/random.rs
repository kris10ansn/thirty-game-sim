use crate::strategy::Strategy;
use rand::Rng;

pub struct RandomStrategy {}
impl Strategy for RandomStrategy {
    fn pick_dice(&self, dice: &Vec<u8>) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        return vec![dice[rng.gen_range(0..dice.len())]];
    }
}
