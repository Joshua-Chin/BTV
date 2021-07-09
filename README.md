# BTV Solver
This program computes optimal ability setups for the BillyTV dice based web game.

## The Game
In the BTV minigame, you start with a fixed number of points to buy abilities with.
Each ability corresponds to a single die and can only be rolled once.
These abilities can be used to attempt challenges.

Each challenge has a target (the minimum to pass the challenge) and a maximum ability count.
To attempt the challenge, select a number of abilities and roll the corresponding dice.
If the total is greater than the target, then you pass the challenge and recieve a reward.
These rewards can improve your dice, increase the ability limit, or help you in the finale.
Challenges can be attempted in any order, and can be retried if you fail.

After defeating all 11 challenges, you roll your remaining abilities in the grand finale.
Your total score is the result of that roll.

## The API
The program expects a file passed via the `input` flag.
This file should contain a list of challenges.
For example:

```
  Paperwork Montage  	  Target: 70  	  Max Abilities: 5
Reward: +1 Diction Strength
  Eating TACOs  	  Target: 40  	  Max Abilities: 4
Reward: One additional Attempt of your most powerful type in Finale
```

The program will then output the a set of configurations to the terminal.
Each configuration contains the challenge order and ability setup that maximizes the chances of
passing all the challenges, given a fixed point budget.