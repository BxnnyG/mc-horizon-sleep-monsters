// Once a frame: watch for the bed's "monsters nearby" message.
fn closet_frame(game: &mut crate::game::Game<'_>) {
    crate::game::closet::frame(game);
}
