//! Core pet engine for Milestone 1

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Mood {
    Happy,
    Neutral,
    Sad,
    Sleeping,
    Scared,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PetState {
    pub mood: Mood,
    pub energy: i32,
    pub xp: u32,
    pub level: u32,
}

impl PetState {
    pub fn new() -> Self {
        Self {
            mood: Mood::Neutral,
            energy: 50,
            xp: 0,
            level: 1,
        }
    }

    pub fn add_xp(&mut self, delta: u32) {
        self.xp = self.xp.saturating_add(delta);
        let new_level = (self.xp / 100) + 1;
        if new_level != self.level {
            self.level = new_level;
        }
    }

    pub fn apply_event(&mut self, event: Event) {
        match event {
            Event::Commit => {
                self.add_xp(10);
                self.mood = Mood::Happy;
                self.energy = (self.energy + 1).min(100);
            }
            Event::TestPass => {
                self.add_xp(15);
                self.mood = Mood::Happy;
            }
            Event::TestFail => {
                self.add_xp(2);
                self.mood = Mood::Sad;
            }
            Event::MergeConflict => {
                self.mood = Mood::Scared;
            }
            Event::Inactivity => {
                self.energy = (self.energy - 5).max(0);
                // if energy drops below 20 change mood to Sad, otherwise stay Neutral
                if self.energy <= 20 {
                    self.mood = Mood::Sad;
                } else {
                    self.mood = Mood::Neutral;
                }
            }
            Event::NpmInstall => {
                self.add_xp(1);
                self.mood = Mood::Neutral;
            }
            Event::ForcePushMain => {
                self.mood = Mood::Scared;
            }
            Event::FridayDeploy => {
                self.add_xp(20);
                self.mood = Mood::Scared;
            }
            Event::BugFix => {
                self.add_xp(12);
                self.mood = Mood::Happy;
            }
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Commit,
    TestPass,
    TestFail,
    MergeConflict,
    Inactivity,
    NpmInstall,
    ForcePushMain,
    FridayDeploy,
    BugFix,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_increases_xp_and_sets_happy() {
        let mut pet = PetState::new();
        pet.apply_event(Event::Commit);
        assert_eq!(pet.xp, 10);
        assert_eq!(pet.mood, Mood::Happy);
    }

    #[test]
    fn test_pass_adds_15_xp() {
        let mut pet = PetState::new();
        pet.apply_event(Event::TestPass);
        assert_eq!(pet.xp, 15);
        assert_eq!(pet.mood, Mood::Happy);
    }

    #[test]
    fn test_fail_adds_small_xp_and_sad() {
        let mut pet = PetState::new();
        pet.apply_event(Event::TestFail);
        assert_eq!(pet.xp, 2);
        assert_eq!(pet.mood, Mood::Sad);
    }

    #[test]
    fn level_up_based_on_xp() {
        let mut pet = PetState::new();
        pet.add_xp(250);
        // 250 / 100 = 2 -> level = 3
        assert_eq!(pet.level, 3);
    }

    #[test]
    fn inactivity_decreases_energy_and_makes_sad() {
        let mut pet = PetState::new();
        pet.energy = 25;
        pet.apply_event(Event::Inactivity);
        assert!(pet.energy < 25);
        assert_eq!(pet.mood, Mood::Sad);
    }
}
