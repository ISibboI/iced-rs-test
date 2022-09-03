BUILTIN_ACTION EXPLORE
name Explore

ACTION train_str
name Lift weights
progressive lifting weights
simple_past lifted weights
strength 1.0
duration 1h

QUEST look_around
activation none
completion action_count(2, look_around)
name Figure out where you are
description You woke up on the side of a road in a village unknown to you. Take a look around to figure out where you are.

QUEST_ACTION look_around look_around
name Look around
duration 1h
