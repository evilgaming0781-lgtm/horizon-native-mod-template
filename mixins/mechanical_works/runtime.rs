// Put testable mod logic here. This file is both the standalone Cargo library
// and the injected `crate::game::mechanical_works` module; no generated copy exists.

/// Creates a message for breaking a block with mechanical works
pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[mechanical-works] Breaking block at {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

/// Handles the before-break hook for mechanical works
pub fn before_break(pos: [i32; 3]) {
    eprintln!("{}", block_break_message(pos));
}

/// Creates a message for placing a block with mechanical works
pub fn block_place_message(pos: [i32; 3], block_type: u32) -> String {
    format!(
        "[mechanical-works] Placing block at {}, {}, {} (type: {})",
        pos[0], pos[1], pos[2], block_type
    )
}

/// Handles the before-place hook for mechanical works
pub fn before_place(pos: [i32; 3], block_type: u32) {
    eprintln!("{}", block_place_message(pos, block_type));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[mechanical-works] Breaking block at -3, 64, 12"
        );
    }

    #[test]
    fn place_message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_place_message([-3, 64, 12], 42),
            "[mechanical-works] Placing block at -3, 64, 12 (type: 42)"
        );
    }
}
