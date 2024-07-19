pub mod player_character;

use player_character::player_character::PlayerCharacter;

fn main() {
    let pc = PlayerCharacter::default();
    println!("{:?}", pc);
}
