use crate::rewards::Rewards;

#[derive(Debug, Eq, PartialEq)]
pub struct Challenge {
    pub name: String,
    pub target: u32,
    pub abilities: u32,
    pub reward: Rewards,
}

pub fn parse<T: AsRef<str>>(text: T) -> Option<Vec<Challenge>> {
    let mut output = Vec::with_capacity(11);

    for entry in text.as_ref().lines().collect::<Vec<&str>>().chunks_exact(2) {
        // Parse the first line
        let mut l1 = entry[0].split('\t');
        // Parse name
        let name = l1.next()?.trim().to_string();
        // Parse target
        let target: u32 = l1.next()?.trim().strip_prefix("Target: ")?.parse().ok()?;
        // Parse abilities
        let abilities: u32 = l1
            .next()?
            .trim()
            .strip_prefix("Max Abilities: ")?
            .parse()
            .ok()?;
        // Parse the reward
        let reward: Rewards = entry[1].trim().strip_prefix("Reward: ")?.parse().ok()?;

        output.push(Challenge {
            name,
            target,
            abilities,
            reward,
        });
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("data.txt");
        let result = parse(input);
        assert!(result.is_some());
        let challenges = result.unwrap();
        assert!(challenges.len() == 11);
        assert_eq!(
            challenges[0],
            Challenge {
                name: "Monologue".to_string(),
                target: 70,
                abilities: 15,
                reward: Rewards::DICTION_RANGE
            }
        );
    }
}
