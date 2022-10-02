LOCATION village
name Village
url village.png
events (1.0, rat), (1.0, infected_rat), (0.8, hare), (0.1, find_rat_plague), (0.1, find_wood_cutter)
activation none
deactivation never

QUEST find_all_village
title Find all quests in the village
description Explore the village to find all the available quests. Some might be rare finds, so you have to explore quite a bit.
activation none
failure never
BEGIN
    QUEST_STAGE find_all
    task Find all quests in the village.
    completion and(quest_activated(rat_plague), quest_activated(wood_cutter))
END

EXPLORATION_EVENT find_rat_plague
name Find the healer
progressive finding the healer
simple_past found the healer
currency 0
activation none
deactivation exploration_event_count(1, find_rat_plague)

QUEST rat_plague
title The rat plague
description You talked to the village's healer and she told you that there is an ongoing rat plague that is making people sick. You will be greatly rewarded for each killed infected rat!
activation exploration_event_count(1, find_rat_plague)
failure never
BEGIN
    QUEST_STAGE kill
    task Kill infected rats.
    completion monster_killed_count(10, infected_rat)
END

EXPLORATION_EVENT infected_rat
monster infected_rat
currency 100
activation quest_activated(rat_plague)
deactivation quest_completed(rat_plague)

MONSTER infected_rat
name Infected rat
hitpoints 120.0
activation none
deactivation never

EXPLORATION_EVENT find_wood_cutter
name Find the wood cutter
progressive finding the wood cutter
simple_past found the wood cutter
currency 0
activation none
deactivation exploration_event_count(1, find_wood_cutter)

QUEST wood_cutter
title Chop me some trees
description The village's wood cutter lives in a hut a little further towards the forest. He could use some help with carrying some logs to his cabin.
activation exploration_event_count(1, find_wood_cutter)
failure never
BEGIN
    QUEST_STAGE collect
    task Collect wood from the forest.
    completion exploration_event_count(10, carry_wood)
END

EXPLORATION_EVENT carry_wood
name Carry wood for the wood cutter
progressive carrying wood
simple_past carried wood
currency 10
activation quest_activated(wood_cutter)
deactivation quest_completed(wood_cutter)