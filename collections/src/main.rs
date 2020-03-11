fn main() {
    string_demo();
    hash_demo();
}

struct Team<'a> {
    team_name: String,
    team_mascot: String,
    players: [Option<&'a mut Player<'a>>; 32],
    eligable: bool,
    score: u32,
}

impl Team<'_> {
    fn add_player(&mut self, player: &Player) {
        for (index, &player) in self.players.iter().enumerate() {
            match &player {
                None => self.players[index] = player,
                _ => (),
            }
        }
    }
    fn is_full(&self, lobby: &Lobby) -> bool {
        for x in lobby.min_size..lobby.max_size {
            match self.players[x as usize] {
                None => return false,
                _ => (),
            }
        }
        return true;
    }
    fn new(player: &'static mut Player) -> Team<'static> {
        let mut players = [None; 32];
        players[0] = Some(player);
        Team {
            team_name: "Enter your team name".to_string(),
            team_mascot: "Choose your team mascot".to_string(),
            players: players,
            eligable: false,
            score: 0,
        }
    }
}

struct Player<'a> {
    username: &'a str,
    kills: u32,
    deaths: u32,
    score: u16,
}

impl Player<'_> {
    fn new(username: &'static str) -> Player {
        Player {
            username,
            kills: 0,
            deaths: 0,
            score: 0,
        }
    }
}

struct Lobby<'a> {
    game_type: GameType,
    teams: [Option<Team<'a>>; 2],
    max_size: u8,
    min_size: u8,
}

enum GameType {
    Slayer,
    BigTeamBattles,
    L33t,
}

impl Default for GameType {
    fn default() -> Self {
        GameType::Slayer
    }
}

impl Lobby<'_> {
    fn new(game_type: GameType) -> Lobby<'static> {
        match game_type {
            Slayer => Lobby {
                game_type,
                teams: [None, None],
                max_size: 8,
                min_size: 4,
            },
            BigTeamBattles => Lobby {
                game_type,
                teams: [None, None],
                max_size: 32,
                min_size: 16,
            },
            L33t => Lobby {
                game_type,
                teams: [None, None],
                max_size: 6,
                min_size: 5,
            },
        }
    }
}

impl Default for Lobby<'static> {
    fn default() -> Self {
        Lobby::new(GameType::Slayer)
    }
}

fn init() {}

use std::collections::HashMap;
use std::convert::TryInto;
use std::{thread, time};
fn hash_demo() {
    let lobby = Lobby::default();

    println!("Searching for players.");
    let mut new_player = Player::new("Virtruvius");

    for team in lobby.teams.iter() {
        match team {
            Some(team) => {
                if !team.is_full(&lobby) {
                    &team.add_player(&mut new_player);
                }
            }
            None => {
                let mut new_team = Team::new(&mut new_player);
            }
        }
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 3);
    scores.insert(String::from("Yellow"), 50);

    let late_additions = vec!["Green", "Red", "Orange"];
    let initial_scores = vec![10; late_additions.len()];
    let late_additions = late_additions.iter().zip(initial_scores.iter());
    for (team, score) in late_additions {
        scores.insert(team.to_string(), *score);
    }
    let (team_count, avg_score, lowest_scorer) = summarize_scores(&scores);
    println!(
        "There are {} teams and the average score is {}.",
        team_count, avg_score
    );

    match scores.get(&lowest_scorer) {
        Some(score) => println!(
            "The lowest scoring team was {} with {} points.",
            lowest_scorer, score
        ),
        _ => (),
    };
}

fn summarize_scores(map: &HashMap<String, u32>) -> (u32, f32, String) {
    let mut team_count: u32 = 0;
    let mut sum_scores: f32 = 0.0;
    let mut lowest_score: u32 = std::u32::MAX;
    let mut lowest_scorer: String = String::new();
    for (team, score) in map.iter() {
        team_count += 1;
        sum_scores += *score as f32;
        if score < &lowest_score {
            lowest_score = *score; //This is just to prove a point after return
            lowest_scorer = team.clone(); //You could return the lowest score and the team
                                          //in the same operation of course.
        }
    }
    (team_count, sum_scores / team_count as f32, lowest_scorer)
}

fn string_demo() {
    let mut summary = "This is".to_string();
    summary.push_str(" a test.");
    summary = summary + " A battle of fates.";
    summary.pop();
    summary.push('!');
    let addition = " Only the able will survive.";
    let summary = summary + &addition; //if assigned to something other than summary
                                       //this won't compile since summary in the add
                                       //opperator is borrowed and thrown out of scope
                                       //afterwards.

    let late_addition = " Who will prevail?";
    let mut latest = summary.clone() + &late_addition; //If you want to keep summary you
                                                       //have to clone first

    let small_edition_that_the_copywriter_snuck_in = " Click here to find out.";
    latest = format!("{}{}", latest, small_edition_that_the_copywriter_snuck_in);
    //you can also use format to avoid borrowing a variable. I assume it copies or uses two refrences

    let mut final_copy = &latest[0..=33]; //When the CEO want's something pithier not
                                          //Not  safe depending on char boundaries and
                                          //langugage differences

    let mut final_copy = String::new(); //CEO thinks accessibility is important and you agree
    let mut sentance_count = 0;
    let sentance_limit = 3;
    for c in latest.chars() {
        //Look for setnance boundaries by char
        //This version is safe in different languages
        if ".?!".contains(c) {
            sentance_count += 1;
        }
        if sentance_count == sentance_limit {
            final_copy.push(c);
            break;
        } else {
            final_copy.push(c);
        }
    }
    println!("{}", final_copy);
}
