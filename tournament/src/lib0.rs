use std::{collections::HashMap, fmt::format};

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default)]
struct Team {
    name: String,
    matches: u8,
    wins: u8,
    losses: u8,
    draws: u8,
    points: u16,
}

impl Team {
    fn win(&mut self) {
        self.wins += 1;
        self.matches += 1;
        self.points += 3;
    }

    fn loss(&mut self) {
        self.losses += 1;
        self.matches += 1;
    }

    fn draw(&mut self) {
        self.draws += 1;
        self.matches += 1;
        self.points += 1;
    }

    fn add_match(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Loss => self.loss(),
            MatchResult::Draw => self.draw(),
        }
    }
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl From<&str> for MatchResult {
    fn from(action: &str) -> MatchResult {
        match action {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            _ => panic!(),
        }
    }
}
impl MatchResult {
    fn reverse(&self) -> MatchResult {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
}

fn tally(match_results: &str) -> String {
    let mut matches_values: HashMap<String, Team> = HashMap::new();

    match_results.lines().for_each(|line| {
        let frags: Vec<&str> = line.split(";").collect();
        let main = frags[0];
        let second = frags[1];
        let action: MatchResult = frags[2].into();

        matches_values
            .entry(main.into())
            .or_insert(Team::new(main.into()))
            .add_match(&action);

        matches_values
            .entry(second.into())
            .or_insert(Team::new(second.into()))
            .add_match(&action.reverse());
    });

    let mut score_values: Vec<&Team> = matches_values.values().collect();
    score_values.sort_by(|a, b| a.points.cmp(&b.points).then_with(|| a.name.cmp(&b.name)));

    vec![String::from(HEADER)]
        .into_iter()
        .chain(score_values.into_iter().map(|a| a.into()))
        .collect::<Vec<String>>()
        .join("\n")
}

impl From<&Team> for String {
    fn from(team: &Team) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team.name, team.matches, team.wins, team.draws, team.losses, team.points
        )
    }
}
