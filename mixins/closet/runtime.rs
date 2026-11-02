// Sleep Monsters for Horizon: when a bed refuses with "You may not
// rest now; there are monsters nearby", the monsters within 8 blocks (5 up
// or down) glow for three seconds (Monsters in the Closet Reborn, MIT, by
// KoroWin and Minenash; its client half, which works on any server).
use super::Game;
use std::cell::RefCell;
use std::time::{Duration, Instant};

pub const ID: &str = "closet";
const KEY: &str = "block.minecraft.bed.not_safe";
const FALLBACK: &str = "You may not rest now; there are monsters nearby";
/// 60 ticks.
const GLOW: Duration = Duration::from_secs(3);

#[derive(Default)]
struct State {
    glowing: Vec<u32>,
    until: Option<Instant>,
    /// The message already answered (its time), so it counts once.
    seen_overlay: bool,
    seen_message: Option<Instant>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub(super) fn glows(id: u32) -> bool {
    STATE.with(|s| {
        let s = s.borrow();
        s.until.is_some_and(|t| t.elapsed() < GLOW) && s.glowing.contains(&id)
    })
}

pub(super) fn frame(game: &mut Game<'_>) {
    if !game.native_on(ID) || !game.native_window() {
        return;
    }
    let refused = game.translate(KEY, FALLBACK);
    // From a server: the action bar, as it arrives (its 60 ticks not yet counted).
    let overlay = game.titles.overlay_time == 60
        && game.titles.overlay.as_ref().is_some_and(|t| t.plain() == refused);
    let fresh_overlay = STATE.with(|s| {
        let mut s = s.borrow_mut();
        let fresh = overlay && !s.seen_overlay;
        s.seen_overlay = overlay;
        fresh
    });
    // In a world of the game's own: the message over the hotbar.
    let fresh_message = match &game.message {
        Some((text, at)) if *text == refused => STATE.with(|s| {
            let mut s = s.borrow_mut();
            let fresh = s.seen_message != Some(*at);
            s.seen_message = Some(*at);
            fresh
        }),
        _ => false,
    };
    if !fresh_overlay && !fresh_message {
        return;
    }
    let Some(p) = game.player.as_ref() else {
        return;
    };
    // Vec3.atBottomCenterOf(player.blockPosition()), and the AABB around it.
    let (x, y, z) = (
        p.position.x.floor() + 0.5,
        p.position.y.floor(),
        p.position.z.floor() + 0.5,
    );
    let list: Vec<u32> = game
        .mobs
        .iter()
        .filter(|m| {
            horizon_client::mobs::ai::is_monster(m.species())
                && (m.position.x - x).abs() < 8.0
                && (m.position.y - y).abs() < 5.0
                && (m.position.z - z).abs() < 8.0
        })
        .map(|m| m.id)
        .collect();
    if !list.is_empty() {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            s.glowing = list;
            s.until = Some(Instant::now());
        });
    }
}
