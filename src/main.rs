mod infrastructure;

use infrastructure::database::get_pokemon;

fn main() {
    let pokemon = get_pokemon().expect("Failed to get pokemon");
    for p in pokemon {
        println!("{:?}", p);
    }
}
