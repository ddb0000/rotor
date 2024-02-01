use rand::Rng;

pub struct Quest {
    pub description: String,
    pub objective: QuestObjective,
    pub reward: QuestReward,
}

pub enum QuestObjective {
    ReachLocation { x: i32, y: i32 },
    DefeatEnemies { enemy_type: String, count: i32 },
    CollectItems { item_type: String, count: i32 },
    // Add more objectives as needed
}

pub struct QuestReward {
    pub experience: i32,
    pub items: Vec<String>, // Names of the reward items
}

impl Quest {
    pub fn new() -> Self {
        // Example quest generation logic
        let mut rng = rand::thread_rng();
        let description = String::from("A newly generated quest.");
        let objective = QuestObjective::ReachLocation { x: rng.gen(), y: rng.gen() };
        let reward = QuestReward {
            experience: 100,
            items: vec![String::from("Health Potion")],
        };

        Quest {
            description,
            objective,
            reward,
        }
    }

    // Add methods for quest completion, tracking, etc.
}
