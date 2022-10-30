use game_of_life::Game;



fn main() {
    let mut game = Game::new(100, 100, 500, 500);
    game.print_window();
}
