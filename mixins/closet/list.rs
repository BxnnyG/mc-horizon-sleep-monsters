// The mod's row in Options > Mods.
fn closet_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: crate::game::closet::ID.into(),
        title: "Sleep Monsters 1.0.0".into(),
        description: "When a bed says there are monsters nearby, those monsters glow for three seconds.".into(),
        keys: Vec::new(),
    });
}
