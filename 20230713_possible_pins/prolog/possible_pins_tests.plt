:- begin_tests(possible_pins).

    test(possible_pins_for_one_number, condition(true)) :-
        get_pins("8", Pins),
        member("5", Pins),
        member("7", Pins),
        member("8", Pins),
        member("8", Pins),
        member("0", Pins), !.

    test(possible_pins_for_two_numbers, condition(true)) :-
        get_pins("11", Pins),
        member("11", Pins),
        member("22", Pins),
        member("44", Pins),
        member("12", Pins),
        member("21", Pins),
        member("14", Pins),
        member("41", Pins),
        member("24", Pins),
        member("42", Pins), !.

    test(possible_pins_for_three_numbers, condition(true)) :-
        get_pins("369", Pins),
        member("339", Pins),
				member("366", Pins),
				member("399", Pins),
				member("658", Pins),
				member("636", Pins),
				member("258", Pins),
				member("268", Pins),
				member("669", Pins),
        member("668", Pins),
				member("266", Pins),
				member("369", Pins),
				member("398", Pins),
				member("256", Pins),
				member("296", Pins),
				member("259", Pins),
				member("368", Pins),
        member("638", Pins),
				member("396", Pins),
				member("238", Pins),
				member("356", Pins),
				member("659", Pins),
				member("639", Pins),
				member("666", Pins),
				member("359", Pins),
        member("336", Pins),
				member("299", Pins),
				member("338", Pins),
				member("696", Pins),
				member("269", Pins),
				member("358", Pins),
				member("656", Pins),
				member("698", Pins),
        member("699", Pins),
				member("298", Pins),
				member("236", Pins),
				member("239", Pins), !.

:- end_tests(possible_pins).

