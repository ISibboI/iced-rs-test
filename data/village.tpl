LOCATION village
name Village
url village.png
events (1.0, rat), (1.0, infected_rat), (0.8, hare), (0.1, find_rat_plague)
activation none
deactivation never

QUEST find_all_village
title Find all quests in the village
description Explore the village to find all the available quests. Some might be rare finds, so you have to explore quite a bit.
activation none
completion and(quest_activated(rat_plague))
failure never

EXPLORATION_EVENT find_rat_plague
name Find the healer
progressive finding the healer
simple_past found the healer
currency 0
activation none
deactivation exploration_event_count(1, find_rat_plague)

QUEST rat_plague
title The rat plague
description You talked to the villages healer and she told you that there is an ongoing rat plague that is making people sick. You will be greatly rewarded for each killed infected rat!
activation exploration_event_count(1, find_rat_plague)
completion monster_killed_count(10, infected_rat)
failure never

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