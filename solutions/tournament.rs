pub struct Stats {
    played: u64,
    win: u64,
    draw: u64,
    lost: u64,
}

impl Default for Stats {
    fn default() -> Stats {
        Stats::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            played: 0,
            win: 0,
            draw: 0,
            lost: 0,
        }
    }
    pub fn score(&self) -> u64 {
        self.win * 3 + self.draw
    }
}

pub fn tally<'a>(matches: &'a str) -> String {
    use std::collections::HashMap;

    let mut results: HashMap<&str, Stats> = HashMap::new();

    for l in matches.lines() {
        let mut line_parts = l.split(';');

        let team_a = line_parts.next().unwrap();
        let team_b = line_parts.next().unwrap();
        let match_res = line_parts.next().unwrap();

        {
            let stats = results.entry(team_a).or_default();
            stats.played += 1;
            match match_res {
                "win" => stats.win += 1,
                "draw" => stats.draw += 1,
                "loss" => stats.lost += 1,
                r => unimplemented!("{}", r),
            }
        }

        {
            let stats = results.entry(team_b).or_default();
            stats.played += 1;
            match match_res {
                "win" => stats.lost += 1,
                "draw" => stats.draw += 1,
                "loss" => stats.win += 1,
                _ => unimplemented!(),
            }
        }
    }

    let mut teams_stats = results.into_iter().collect::<Vec<_>>();
    teams_stats.sort_by_key(|(n, s)| -> (i128, &'a str) { (-i128::from(s.score()), n) });

    let mut rows = Vec::with_capacity(teams_stats.len() + 1);
    rows.push("Team                           | MP |  W |  D |  L |  P".to_string());
    rows.extend(teams_stats.into_iter().map(|(team, stats)| {
        format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            team,
            stats.played,
            stats.win,
            stats.draw,
            stats.lost,
            stats.score()
        )
    }));

    rows.join("\n")
}
