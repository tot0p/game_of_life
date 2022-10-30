use game_of_life::Game;



fn main() {
    let mut game = Game::new(100, 100, 500, 500);
    game.generate();
    game.print_window();
}
