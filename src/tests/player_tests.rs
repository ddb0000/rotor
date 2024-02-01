#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement() {
        let mut player = Player::new();
        player.move_player(Direction::Up);
        assert_eq!(player.position.y, -1.0); // Assuming starting y position is 0.0
    }

    // Add more tests for player, weapon, map, quest, etc.
}
