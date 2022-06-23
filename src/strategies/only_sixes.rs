use crate::strategy::Strategy;

pub struct OnlySixesStrategy {}
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
