use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug)]
struct Teams {
    teams: HashMap<String, Team>,
}

impl Teams {
    pub fn sorted(&self) -> Vec<Team> {
        let mut teams: Vec<Team> = self.teams.values().cloned().collect();
        teams.sort_by(|a, b| match a.points().cmp(&b.points()).reverse() {
            std::cmp::Ordering::Equal => a.name.cmp(&b.name),
            other => other,
        });
        teams
    }
}

#[derive(Debug, Clone)]
struct Team {
    name: String,
    matches: Vec<Match>,
}

#[derive(Debug, Clone)]
enum Match {
    Win,
    Lose,
    Tie,
}

impl Team {
    pub fn new(name: String) -> Self {
        Self {
            name,
            matches: vec![],
        }
    }

    pub fn add_match(&mut self, played_match: Match) {
        self.matches.push(played_match)
    }

    pub fn points(&self) -> usize {
        self.wins() * 3 + self.ties()
    }

    pub fn wins(&self) -> usize {
        self.matches
            .iter()
            .filter(|item| matches!(item, Match::Win))
            .count()
    }
    pub fn losses(&self) -> usize {
        self.matches
            .iter()
            .filter(|item| matches!(item, Match::Lose))
            .count()
    }
    pub fn ties(&self) -> usize {
        self.matches
            .iter()
            .filter(|item| matches!(item, Match::Tie))
            .count()
    }

    pub fn matches_played(&self) -> usize {
        self.matches.len()
    }

    pub fn info(&self) -> String {
        format!(
            "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            self.name,
            self.matches_played(),
            self.wins(),
            self.ties(),
            self.losses(),
            self.points()
        )
    }
}

fn header() -> String {
    let header = format!(
        "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
        "Team", "MP", "W", "D", "L", "P"
    );
    println!("{}", header);
    header
}

fn parse(input: String) -> Teams {
    let parts: Vec<(&str, &str, Match)> = input
        .lines()
        .map(|line| {
            let mut components = line.split(';');
            let team_a = components.next().unwrap().trim();
            let team_b = components.next().unwrap().trim();

            let winner = match components.next().unwrap().trim() {
                "win" => Match::Win,
                "loss" => Match::Lose,
                "draw" => Match::Tie,
                _ => panic!("should not happen"),
            };
            (team_a, team_b, winner)
        })
        .collect();
    let mut teams_map = Teams {
        teams: HashMap::new(),
    };

    for part in parts {
        let (team_a_name, team_b_name, match_status) = part;

        let team_a = teams_map
            .teams
            .entry(team_a_name.to_string())
            .or_insert_with(|| Team::new(team_a_name.to_string()));

        team_a.add_match(match_status.clone());

        let team_b = teams_map
            .teams
            .entry(team_b_name.to_string())
            .or_insert_with(|| Team::new(team_b_name.to_string()));

        team_b.add_match(match match_status {
            Match::Win => Match::Lose,
            Match::Lose => Match::Win,
            Match::Tie => Match::Tie,
        });
    }
    teams_map
}

pub fn tally(match_results: &str) -> String {
    let teams = parse(match_results.to_string());
    let mut result = vec![header()];
    for team in teams.sorted() {
        result.push(team.info())
    }

    result.join("\n")
}
