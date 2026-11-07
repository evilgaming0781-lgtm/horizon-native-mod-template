// Runs before the native method. Returning unit leaves its body/result intact.
fn before_place(pos: [i32; 3], block_type: u32) {
    crate::game::mechanical_works::before_place(pos, block_type);
}