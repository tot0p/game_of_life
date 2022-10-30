use game_of_life::Game;


fn main() {
    let mut game = Game::new(10, 10);
    game.generate();
    for _ in 0..10 {
        game.update();
        game.print();
        println!("");
    }
}
