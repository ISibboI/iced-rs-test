INITIALISATION
starting_location village

BUILTIN_ACTION WAIT
name Wait
progressive waiting
simple_past waited
activation none
deactivation never

BUILTIN_ACTION SLEEP
name Sleep
progressive sleeping
simple_past slept
activation none
deactivation never

BUILTIN_ACTION TAVERN
name Tavern
progressive relaxing in the tavern
simple_past relaxed in the tavern
activation none
deactivation never

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

LOCATION village
name Village
events (1.0, find_money), (0.1, find_more_money)
activation none
deactivation never

EXPLORATION_EVENT find_money
name Find money
progressive finding money
simple_past found money

EXPLORATION_EVENT find_more_money
name Find money
progressive finding money
simple_past found money

MONSTER rat
name Rat