
# Definitions

- *Universally Unique Identifier (UUIDs)* - an assigned 64-bit random(ish) value that is unpredictable. 
    - In consumer products, *serial number* is often used as a unique identifier for a unit. `Serial` implies in a sequence, which would be predictable, so we use `UUID` instead.

- *Checkpoint* - a geo-coded timestamp on the history of a UUID. Each `checkpoint` may include additional references such as documents, images, videos, technical values like test results, etc. 

- *New UUID* - when a new UUID is created, it is given a parent UUID to represent the source or literal parent of a live plant. 
    - The only exception to this is when creating a new vendor UUID.
    - Vendors -> Seeds -> Germinates -> Mother -> etc etc.

![image](https://user-images.githubusercontent.com/32852271/149630317-d3c9ece5-e389-42ca-b0b8-619a1276fb0c.png)

```
@startuml
digraph G {
    rankdir=TB;
    notes [label="- Purple lines are checkpoints\n- Gold lines are new UUIDs\n- +n are new UUIDs\n- -n are scrapped UUIDs",shape=note];
    node [shape=record];

    seeds_from_cookies [label="Seeds\n|{UUIDs|Pieces}|{{1}|{100}}"];
    cubed [label="Cubed\n|{UUIDs|Pieces}|{{75}|{75}}"];
    did_not_germinate [label="Did not Germinate\n|{UUIDs|Pieces}|{{1}|{25}}"];
    discard [fillcolor=pink,style=filled,label="Discard\n|{UUIDs|Pieces}|{{1}|{var}}"];
    mother_plants [label="Mother Plants\n|{UUIDs|Pieces}|{{55}|{55}}"];
    clones [label="Clones\n|{UUIDs|Pieces}|{{375}|{375}}"];
    clone_buyers [fillcolor=darkseagreen1,style=filled,label="Clone Buyers\n|{UUIDs|Pieces}|{{75}|{75}}"];
    flower [label="Flower\n|{UUIDs|Pieces}|{{1,783}|{1,783}}"];
    dispensaries [label="Dispensaries\n|{UUIDs|Pieces}|{{1,505}|{1,505}}"];
    consumers [label="Consumer Scan\n|{UUIDs|Pieces}|{{?}|{?}}"];
    pos_scan [fillcolor=darkseagreen1,style=filled,label="PoS Scan\n|{UUIDs|Pieces}|{{1,273}|{1,273}}"];

    seeds_from_cookies -> cubed [label="+ 75 uuids",color=gold4];
    cubed -> mother_plants [label="",color=purple];
    cubed -> discard [label=" -20",color=purple];

    seeds_from_cookies -> did_not_germinate [label="",color=purple];
    did_not_germinate -> discard [label=" (never became a UUID)",color=purple];

    mother_plants -> clones [color=gold4,label=" + 320"];
    mother_plants -> discard [label="-7",color=purple];

    clones -> flower [label=" + 1,531",color=gold4];
    clones -> clone_buyers [color=purple];
    clones -> discard [label="-123",color=purple];

    flower -> dispensaries [label="",color=purple];
    flower -> discard [label="-278",color=purple];

    dispensaries -> consumers [label="",color=purple];
    dispensaries -> discard [label="-232",color=purple];

    consumers -> pos_scan [label="",color=purple];
    dispensaries -> pos_scan [label=" Some portion of UUIDs will be scanned by consumers\n and all should have point-of-sale scan",color=purple];
}
@enduml
```