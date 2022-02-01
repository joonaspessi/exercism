use std::{fmt::format, vec};

#[derive(Debug)]
struct Team {
    name: String,
    matches: Vec<Match>,
}

#[derive(Debug)]
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

fn parse(input: String) -> Vec<Team> {
    let parts: Vec<(&str, &str, bool)> = input
        .lines()
        .map(|line| {
            let mut components = line.split(';');
            let team_a = components.next().unwrap().trim();
            let team_b = components.next().unwrap().trim();
            let winner = matches!(components.next().unwrap().trim(), "win");
            (team_a, team_b, winner)
        })
        .collect();

    let mut teams = vec![];
    for part in parts {
        let (team_a_name, team_b_name, is_a_winner) = part;

        let team_a = Team {
            name: team_a_name.to_string(),
            matches: vec![if is_a_winner { Match::Win } else { Match::Lose }],
        };

        let team_b = Team {
            name: team_b_name.to_string(),
            matches: vec![if !is_a_winner {
                Match::Win
            } else {
                Match::Lose
            }],
        };

        teams.push(team_a);
        teams.push(team_b);
    }
    teams
}

pub fn tally(match_results: &str) -> String {
    let teams = parse(match_results.to_string());
    let mut result = vec![header()];
    for team in teams {
        result.push(team.info())
    }

    result.join("\n")
}
