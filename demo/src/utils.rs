use crate::demo::PlayerData;
use std::iter::Sum;

pub fn get_bitfield_from_indices(indices: impl Iterator<Item = usize>) -> i32 {
    i32::sum(indices.map(|x| 2_i32.pow((x - 1) as u32)))
}

pub fn get_voice_filter<'a, I>(players: I) -> i32
where
    I: Iterator<Item = &'a PlayerData> + Clone,
{
    let player_slots = players.map(|p| p.slot);

    get_bitfield_from_indices(player_slots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitfield_from_indices() {
        assert_eq!(get_bitfield_from_indices([1, 2, 3].into_iter()), 7);
        assert_eq!(get_bitfield_from_indices([4, 5].into_iter()), 24);
        assert_eq!(get_bitfield_from_indices([10].into_iter()), 512);
        assert_eq!(get_bitfield_from_indices([].into_iter()), 0);
    }
}
