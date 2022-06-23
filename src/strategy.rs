pub trait Strategy {
    fn pick_dice(&self, dice: &Vec<u8>) -> Vec<u8>;
}
