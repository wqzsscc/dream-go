#!/usr/bin/env python3

from test_suite import *

with TestCase('Dead Dragon 2') as dead_dragon_2:
    #
    #      a b c d e f g h j k l m n o p q r s t
    #    ╭───────────────────────────────────────╮
    # 19 │             ● ● ○ ○ ○ ○ ○     ○   ○   │ 19
    # 18 │       ● ● ●   ○ ● ○ ● ○ ○ ○ ○ ○ ○ ● ● │ 18
    # 17 │     ● ● ○ ● ○   ● ○ ● ● ● ● ● ○ ●   ● │ 17
    # 16 │ ● ● ● ○ ○ ○ ● ●   ● ●   ● ○ ○ ○ ● ● ● │ 16
    # 15 │ ○ ● ○   ○ ○ ○ ○ ● ●     ● ○ ○ ●   ●   │ 15
    # 14 │ ○ ○ ○   ● ○ ● ○ ○ ● ●     ● ● ●       │ 14
    # 13 │         ● ●   ○   ○ ● ● ● ●       ● ● │ 13
    # 12 │                   ○ ● ○ ○   ● ●   ●   │ 12
    # 11 │     ○           ○ ○ ● ● ○ ○ ○ ● ● ● ○ │ 11
    # 10 │   ● ○ ○   ○ ○ ○ ● ○ ○ ● ○     ○ ○ ○   │ 10
    #  9 │   ○     ○     ○ ● ● ● ● ● ○ ○ ● ●     │ 9
    #  8 │     ○   ○ ○     ○   ● ○ ○ ○   ○ ● ○   │ 8
    #  7 │ ○   ○       ○ ○ ○ ○ ○ ● ● ○   ○ ○     │ 7
    #  6 │ ● ○ ○ ○ ○ ○ ● ○ ● ● ● ●   ● ○         │ 6
    #  5 │ ● ● ● ○ ●   ● ○ ●     ● ● ● ○ ● ●     │ 5
    #  4 │     ● ● ● ● ● ● ● ●   ● ○ ○ ○ ○ ●     │ 4
    #  3 │ ● ●   ● ○ ● ○ ○ ○ ● ● ○   ○   ○ ○ ○   │ 3
    #  2 │   ● ○ ○ ○ ○ ○ ● ● ● ○ ○ ○     ○       │ 2
    #  1 │ ● ○ ○ ○     ○ ○ ● ● ○   ● ○   ○       │ 1
    #    ╰───────────────────────────────────────╯
    #      a b c d e f g h j k l m n o p q r s t
    #     ● Black    ○ White
    #
    dead_dragon_2.setup_from_sgf('examples/dead_dragon_2.sgf', 722)
    dead_dragon_2.final_score().expect_to(be_winner('B', '4.5'))
