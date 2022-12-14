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

QUEST multistage_test
title Multistage test
description You woke up on the side of a road in a village unknown to you.
activation none
failure never
BEGIN
    QUEST_STAGE look_around_1
    task Take a look around to figure out where you are.
    completion action_count(1, train_str)

    QUEST_STAGE look_around_2
    task Second.
    completion action_count(1, train_str)

    QUEST_STAGE look_around_3
    task Third a b dfg  sef rtca rcartr rrtr.
    completion action_count(1, train_str)
END

QUEST look_around
title Figure out where you are
description You woke up on the side of a road in a village unknown to you.
activation none
failure never
BEGIN
    QUEST_STAGE look_around
    task Take a look around to figure out where you are.
    completion action_count(2, look_around)
END

QUEST_STAGE_ACTION look_around
quest look_around
quest_stage look_around
name Look around
progressive looking around
simple_past looked around
type WORK
duration 1h
currency 0

LOCATION farms
name Farms
events (0.5, rat), (0.8, hare), (0.2, dog)
activation explore_count(10, village)
deactivation never

LOCATION forest
name Forest
events (0.1, dog), (0.3, wolf), (1.0, deer), (1.0, forest_nothing), (0.5, carry_wood)
activation or(monster_killed_count(10, dog), quest_activated(wood_cutter))
deactivation never

EXPLORATION_EVENT forest_nothing
name Find nothing in the forest
progressive finding nothing
simple_past found nothing
currency 0
activation none
deactivation never

EXPLORATION_EVENT rat
currency 1
monster rat
activation none
deactivation never

EXPLORATION_EVENT hare
currency 10
monster hare
activation none
deactivation never

EXPLORATION_EVENT dog
currency 13
monster dog
activation none
deactivation never

EXPLORATION_EVENT deer
currency 20
monster deer
activation none
deactivation never

EXPLORATION_EVENT wolf
currency 20
monster wolf
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

MONSTER deer
name Deer
hitpoints 800.0
activation level_geq(6)
deactivation never

MONSTER wolf
name Wolf
hitpoints 1300.0
activation level_geq(8)
deactivation never