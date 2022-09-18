INITIALISATION
starting_location village
starting_time 5000y+120d

BUILTIN_ACTION WAIT
name Wait
progressive waiting
simple_past waited
duration 1h
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
duration 1h
activation none
deactivation never

BUILTIN_ACTION EXPLORE
name Explore
progressive exploring
simple_past explored
duration 1h
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
events (1.0, find_money), (0.8, find_more_money)
activation none
deactivation never

EXPLORATION_EVENT find_money
name Find money
progressive finding money
simple_past found money
currency 3
monster rat
activation none
deactivation never

EXPLORATION_EVENT find_more_money
name Find money
progressive finding money
simple_past found money
currency 10
monster hare
activation none
deactivation never

MONSTER rat
name Rat
hitpoints 60.0
activation none
deactivation never

MONSTER hare
name Hare
hitpoints 150.0
activation level_geq(2)
deactivation never

MONSTER dog
name Dog
hitpoints 300.0
activation level_geq(4)
deactivation never