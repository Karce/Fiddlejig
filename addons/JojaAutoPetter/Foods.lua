-- Foods.lua — item -> diet map for Joja Mart's Auto-Petter.
--
-- The game exposes the pet's diet (GetPetFoodTypes) but NOT a given food item's
-- diet, so we need a lookup table. Each entry is itemID -> diet category, where
-- the category strings match what GetPetFoodTypes() returns on an enUS client
-- ("Meat", "Fish", "Bread", "Cheese", "Fruit", "Fungus").
--
-- NOTE: only the *diet* is stored here. A food's level is read at runtime via
-- GetItemInfo, so this table never goes stale on levels — and a missing or wrong
-- entry is harmless: the game itself refuses food the pet can't eat, so the worst
-- case is "that food isn't auto-selected." Add IDs below as needed.
--
-- Data compiled from community pet-food references for TBC (2.5.x); IDs above the
-- TBC item range are intentionally omitted. The same table serves Classic Era —
-- TBC-only IDs simply never show up in Era bags.

local _, ns = ...

ns.Foods = {
	-- Meat (117)
	[117]='Meat', [723]='Meat', [729]='Meat', [769]='Meat', [1015]='Meat', [1017]='Meat', [1080]='Meat', [1081]='Meat',
	[2287]='Meat', [2672]='Meat', [2673]='Meat', [2677]='Meat', [2679]='Meat', [2680]='Meat', [2681]='Meat', [2684]='Meat',
	[2685]='Meat', [2687]='Meat', [2886]='Meat', [2888]='Meat', [2924]='Meat', [3173]='Meat', [3220]='Meat', [3404]='Meat',
	[3662]='Meat', [3667]='Meat', [3712]='Meat', [3726]='Meat', [3727]='Meat', [3728]='Meat', [3729]='Meat', [3730]='Meat',
	[3731]='Meat', [3770]='Meat', [3771]='Meat', [4457]='Meat', [4599]='Meat', [4739]='Meat', [5051]='Meat', [5465]='Meat',
	[5467]='Meat', [5469]='Meat', [5470]='Meat', [5471]='Meat', [5472]='Meat', [5474]='Meat', [5477]='Meat', [5478]='Meat',
	[5479]='Meat', [5480]='Meat', [6890]='Meat', [7097]='Meat', [8952]='Meat', [9681]='Meat', [11444]='Meat', [12037]='Meat',
	[12184]='Meat', [12202]='Meat', [12203]='Meat', [12204]='Meat', [12205]='Meat', [12208]='Meat', [12209]='Meat', [12210]='Meat',
	[12213]='Meat', [12223]='Meat', [12224]='Meat', [13851]='Meat', [17119]='Meat', [17222]='Meat', [17407]='Meat', [18045]='Meat',
	[19223]='Meat', [19224]='Meat', [19304]='Meat', [19305]='Meat', [19306]='Meat', [19995]='Meat', [20074]='Meat', [20424]='Meat',
	[21023]='Meat', [21024]='Meat', [21235]='Meat', [22644]='Meat', [23495]='Meat', [23676]='Meat', [24105]='Meat', [27635]='Meat',
	[27636]='Meat', [27651]='Meat', [27655]='Meat', [27657]='Meat', [27658]='Meat', [27659]='Meat', [27660]='Meat', [27668]='Meat',
	[27669]='Meat', [27671]='Meat', [27674]='Meat', [27677]='Meat', [27678]='Meat', [27681]='Meat', [27682]='Meat', [27854]='Meat',
	[29292]='Meat', [29451]='Meat', [30610]='Meat', [31670]='Meat', [31671]='Meat', [31672]='Meat', [31673]='Meat', [32685]='Meat',
	[32686]='Meat', [33120]='Meat', [33254]='Meat', [33454]='Meat', [33872]='Meat',

	-- Fish (93)
	[787]='Fish', [1326]='Fish', [2674]='Fish', [2675]='Fish', [2682]='Fish', [4592]='Fish', [4593]='Fish', [4594]='Fish',
	[4603]='Fish', [4655]='Fish', [5095]='Fish', [5468]='Fish', [5476]='Fish', [5503]='Fish', [5504]='Fish', [5527]='Fish',
	[6038]='Fish', [6289]='Fish', [6290]='Fish', [6291]='Fish', [6303]='Fish', [6308]='Fish', [6316]='Fish', [6317]='Fish',
	[6361]='Fish', [6362]='Fish', [6887]='Fish', [6889]='Fish', [7974]='Fish', [8364]='Fish', [8365]='Fish', [8957]='Fish',
	[8959]='Fish', [12206]='Fish', [12207]='Fish', [12216]='Fish', [12238]='Fish', [13546]='Fish', [13754]='Fish', [13755]='Fish',
	[13756]='Fish', [13758]='Fish', [13759]='Fish', [13760]='Fish', [13888]='Fish', [13889]='Fish', [13890]='Fish', [13893]='Fish',
	[13927]='Fish', [13928]='Fish', [13929]='Fish', [13930]='Fish', [13932]='Fish', [13933]='Fish', [13934]='Fish', [13935]='Fish',
	[15924]='Fish', [16766]='Fish', [16971]='Fish', [19996]='Fish', [21071]='Fish', [21072]='Fish', [21153]='Fish', [21217]='Fish',
	[21552]='Fish', [24477]='Fish', [27422]='Fish', [27425]='Fish', [27429]='Fish', [27435]='Fish', [27437]='Fish', [27438]='Fish',
	[27439]='Fish', [27515]='Fish', [27516]='Fish', [27661]='Fish', [27662]='Fish', [27663]='Fish', [27664]='Fish', [27665]='Fish',
	[27666]='Fish', [27667]='Fish', [27858]='Fish', [29452]='Fish', [30155]='Fish', [33004]='Fish', [33048]='Fish', [33052]='Fish',
	[33053]='Fish', [33451]='Fish', [33823]='Fish', [33824]='Fish', [33867]='Fish',

	-- Bread (23)
	[2683]='Bread', [3666]='Bread', [4540]='Bread', [4541]='Bread', [4542]='Bread', [4544]='Bread', [4601]='Bread', [8950]='Bread',
	[13724]='Bread', [16169]='Bread', [17197]='Bread', [19301]='Bread', [19696]='Bread', [20857]='Bread', [23160]='Bread', [24072]='Bread',
	[27855]='Bread', [28486]='Bread', [29394]='Bread', [29449]='Bread', [30816]='Bread', [33449]='Bread', [33924]='Bread',

	-- Cheese (13)
	[414]='Cheese', [422]='Cheese', [1707]='Cheese', [2070]='Cheese', [3665]='Cheese', [3927]='Cheese', [8932]='Cheese', [12218]='Cheese',
	[17406]='Cheese', [27857]='Cheese', [29448]='Cheese', [30458]='Cheese', [33443]='Cheese',

	-- Fruit (22)
	[4536]='Fruit', [4537]='Fruit', [4538]='Fruit', [4539]='Fruit', [4602]='Fruit', [8953]='Fruit', [11950]='Fruit', [13810]='Fruit',
	[16168]='Fruit', [19994]='Fruit', [20031]='Fruit', [20516]='Fruit', [21030]='Fruit', [21031]='Fruit', [21033]='Fruit', [22324]='Fruit',
	[24009]='Fruit', [27856]='Fruit', [28112]='Fruit', [29393]='Fruit', [29450]='Fruit', [32721]='Fruit',

	-- Fungus (14)
	[3448]='Fungus', [4604]='Fungus', [4605]='Fungus', [4606]='Fungus', [4607]='Fungus', [4608]='Fungus', [8948]='Fungus', [24008]='Fungus',
	[24539]='Fungus', [27676]='Fungus', [27859]='Fungus', [29453]='Fungus', [30355]='Fungus', [33452]='Fungus',
}

-- Raw foods (cooking reagents) to feed only as a last resort, so they're saved for
-- leveling Cooking. They stay eligible — cooked food of the same happiness tier is
-- simply preferred (see Better() in Core.lua). This is the soft version of ns.Exclude
-- below, which blocks a food outright.
--
-- Audited against Classic + TBC cooking recipes: every entry is a reagent that a
-- recipe turns into a different cooked food (shown after the arrow). `/joja debug`
-- tags each match "raw", so you can spot anything this table is missing.
ns.Raw = {
	-- Meats
	[723] = true,   -- Goretusk Liver       -> Goretusk Liver Pie
	[729] = true,   -- Stringy Vulture Meat  -> Westfall Stew
	[769] = true,   -- Chunk of Boar Meat    -> Roasted Boar Meat
	[1015] = true,  -- Lean Wolf Flank       -> Seasoned Wolf Kabob
	[1080] = true,  -- Tough Condor Meat     -> Redridge Goulash
	[1081] = true,  -- Crisp Spider Meat     -> Redridge Goulash
	[2672] = true,  -- Stringy Wolf Meat     -> Charred Wolf Meat
	[2673] = true,  -- Coyote Meat           -> Coyote Steak
	[2677] = true,  -- Boar Ribs             -> Succulent Pork Ribs
	[2886] = true,  -- Crag Boar Rib         -> Beer Basted Boar Ribs
	[2924] = true,  -- Crocolisk Meat        -> Crocolisk Steak
	[3173] = true,  -- Bear Meat             -> Smoked Bear Meat
	[3404] = true,  -- Buzzard Wing          -> Barbecued Buzzard Wing
	[3667] = true,  -- Tender Crocolisk Meat -> Crocolisk Gumbo
	[3712] = true,  -- Turtle Meat           -> Soothing Turtle Bisque
	[3730] = true,  -- Big Bear Meat         -> Big Bear Steak
	[3731] = true,  -- Lion Meat             -> Hot Lion Chops
	[5051] = true,  -- Dig Rat               -> Dig Rat Stew
	[5465] = true,  -- Small Spider Leg      -> Kaldorei Spider Kabob
	[5467] = true,  -- Kodo Meat             -> Roasted Kodo Meat
	[5469] = true,  -- Strider Meat          -> Strider Stew
	[5470] = true,  -- Thunder Lizard Tail   -> Crispy Lizard Tail
	[5471] = true,  -- Stag Meat             -> Lean Venison
	[12037] = true, -- Mystery Meat          -> Mystery Stew
	[12184] = true, -- Raptor Flesh          -> Roast Raptor
	[12202] = true, -- Tiger Meat            -> Jungle Stew
	[12203] = true, -- Red Wolf Meat         -> Hot Wolf Ribs
	[12204] = true, -- Heavy Kodo Meat       -> Heavy Kodo Stew
	[12205] = true, -- White Spider Meat     -> Spider Sausage
	[12208] = true, -- Tender Wolf Meat      -> Tender Wolf Steak
	[12223] = true, -- Meaty Bat Wing        -> Crispy Bat Wing
	[20424] = true, -- Sandworm Meat         -> Smoked Desert Dumplings
	[21024] = true, -- Chimaerok Tenderloin  -> Dirge's Kickin' Chimaerok Chops
	[22644] = true, -- Crunchy Spider Leg    -> Crunchy Spider Surprise
	[23676] = true, -- Moongraze Stag Tenderloin -> Roasted Moongraze Tenderloin
	[27668] = true, -- Lynx Meat             -> Lynx Steak
	[27669] = true, -- Bat Flesh             -> Bat Bites
	[27671] = true, -- Buzzard Meat          -> Buzzard Bites
	[27674] = true, -- Ravager Flesh         -> Ravager Dog
	[27677] = true, -- Chunk o' Basilisk     -> Blackened Basilisk
	[27678] = true, -- Clefthoof Meat        -> Roasted Clefthoof
	[27681] = true, -- Warped Flesh          -> Warp Burger
	[27682] = true, -- Talbuk Venison        -> Talbuk Steak
	[31670] = true, -- Raptor Ribs           -> Mok'Nathal Shortribs
	[31671] = true, -- Serpent Flesh         -> Crunchy Serpent

	-- Fish & shellfish
	[2674] = true,  -- Crawler Meat          -> Crab Cake
	[2675] = true,  -- Crawler Claw          -> Cooked Crab Claw
	[4603] = true,  -- Raw Spotted Yellowtail -> Spotted Yellowtail
	[4655] = true,  -- Giant Clam Meat       -> Giant Clam Scorcho
	[5468] = true,  -- Soft Frenzy Flesh     -> Fillet of Frenzy
	[5503] = true,  -- Clam Meat             -> Boiled Clams
	[5504] = true,  -- Tangy Clam Meat       -> Goblin Deviled Clams
	[6289] = true,  -- Raw Longjaw Mud Snapper -> Longjaw Mud Snapper
	[6291] = true,  -- Raw Brilliant Smallfish -> Brilliant Smallfish
	[6303] = true,  -- Raw Slitherskin Mackerel -> Slitherskin Mackerel
	[6308] = true,  -- Raw Bristle Whisker Catfish -> Bristle Whisker Catfish
	[6317] = true,  -- Raw Loch Frenzy       -> Loch Frenzy Delight
	[6361] = true,  -- Raw Rainbow Fin Albacore -> Rainbow Fin Albacore
	[6362] = true,  -- Raw Rockscale Cod     -> Rockscale Cod
	[7974] = true,  -- Zesty Clam Meat       -> Clamlette Magnifique
	[8365] = true,  -- Raw Mithril Head Trout -> Mithril Head Trout
	[8959] = true,  -- Raw Spinefin Halibut  -> Spinefin Halibut
	[12206] = true, -- Tender Crab Meat      -> Spiced Chili Crab
	[13754] = true, -- Raw Glossy Mightfish  -> Cooked Glossy Mightfish
	[13755] = true, -- Winter Squid          -> Grilled Squid
	[13756] = true, -- Raw Summer Bass       -> Hot Smoked Bass
	[13758] = true, -- Raw Redgill           -> Filet of Redgill
	[13759] = true, -- Raw Nightfin Snapper  -> Nightfin Soup
	[13760] = true, -- Raw Sunscale Salmon   -> Poached Sunscale Salmon
	[13888] = true, -- Darkclaw Lobster      -> Lobster Stew
	[13889] = true, -- Raw Whitescale Salmon -> Baked Salmon
	[13893] = true, -- Large Raw Mightfish   -> Mightfish Steak
	[21071] = true, -- Raw Sagefish          -> Smoked Sagefish
	[21153] = true, -- Raw Greater Sagefish  -> Sagefish Delight
	[24477] = true, -- Jaggal Clam Meat      -> Clam Bar
	[27422] = true, -- Barbed Gill Trout     -> Blackened Trout
	[27425] = true, -- Spotted Feltail       -> Feltail Delight
	[27429] = true, -- Zangarian Sporefish   -> Blackened Sporefish
	[27435] = true, -- Figluster's Mudfish   -> Grilled Mudfish
	[27437] = true, -- Icefin Bluefish       -> Poached Bluefish
	[27438] = true, -- Golden Darter         -> Golden Fish Sticks
	[27439] = true, -- Furious Crawdad       -> Spicy Crawdad
	[27515] = true, -- Huge Spotted Feltail  -> Fisherman's Feast
	[27516] = true, -- Enormous Barbed Gill Trout -> Hot Buttered Trout
	[33823] = true, -- Bloodfin Catfish      -> Broiled Bloodfin
	[33824] = true, -- Crescent-Tail Skullfish -> Skullfish Soup

	-- Eggs (mis-tagged as Fish in ns.Foods above, but genuine cooking reagents)
	[6889] = true,  -- Small Egg             -> Herb Baked Egg
	[12207] = true, -- Giant Egg             -> Monster Omelet
}

-- Foods you never want auto-fed at all. Stronger than ns.Raw — these are skipped
-- even when nothing else is available. Add itemIDs here, e.g. [769] = true.
ns.Exclude = {}
