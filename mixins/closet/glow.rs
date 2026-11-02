// A monster that keeps the player awake glows white.
fn closet_glow(m: &horizon_client::mobs::Mob) -> core::ops::ControlFlow<Option<u32>> {
    if crate::game::closet::glows(m.id) {
        core::ops::ControlFlow::Break(Some(0xFF_FFFF))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
