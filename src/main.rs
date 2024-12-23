use connect4_tui::{board::Board, board_cell::BoardCell, player_token::PlayerToken};

fn main() {
    use BoardCell::Empty;
    use BoardCell::Placed as P;

    use PlayerToken::Player1 as P1;
    use PlayerToken::Player2 as P2;

    let board = Board([
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        [Empty, Empty, P(P1), Empty, Empty, Empty, Empty],
        [P(P1), P(P2), P(P1), P(P2), P(P2), P(P1), P(P2)],
    ]);
    println!("{}", board);
}
