INITIALISATION
starting_location village

BUILTIN_ACTION EXPLORE
name Explore
progressive exploring
simple_past explored
activation none
deactivation never

ACTION train_str
name Lift weights
progressive lifting weights
simple_past lifted weights
type TRAIN
duration 1h
strength 1.0
currency 0
activation none
deactivation never

QUEST look_around
title Figure out where you are
description You woke up on the side of a road in a village unknown to you. Take a look around to figure out where you are.
activation none
completion action_count(2, look_around)
failure never

QUEST_ACTION look_around
name Look around
progressive looking around
simple_past looked around
type EXPLORE
duration 1h
currency 0