use game_of_life::Game;



fn main() {
    let mut game = Game::new(50, 50, 500, 500);
    game.print_window();
}
