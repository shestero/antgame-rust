mod ant_state;
mod field;

use field::Field;

fn main() {
    let mut field = Field::create();

    field.play();
    field.save("antway.png".to_string());
}
