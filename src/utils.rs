use demo::demo::PlayerData;
use thaw::BadgeColor;

#[derive(Clone, Copy, PartialEq)]
pub enum Team {
    Even,
    Odd,
}

impl Team {
    pub fn from(player: &PlayerData) -> Self {
        if player.team_number % 2 == 0 {
            Self::Even
        } else {
            Self::Odd
        }
    }

    pub fn has_player(&self, player: &PlayerData) -> bool {
        self == &Self::from(player)
    }

    pub fn get_badge_color(&self) -> BadgeColor {
        match self {
            Self::Even => BadgeColor::Brand,
            Self::Odd => BadgeColor::Danger,
        }
    }

    pub fn get_name(&self) -> Option<&'static str> {
        match self.get_badge_color() {
            BadgeColor::Brand => Some("Team Blue"),
            BadgeColor::Danger => Some("Team Red"),
            _ => None,
        }
    }
}
