pub const TOK_MAP: [(&str,u8);119] = [
    ("close",114),
    ("com_auto",13),
    ("com_clr",12),
    ("com_con",6),
    ("com_del",9),
    ("com_himem",16),
    ("com_load",4),
    ("com_lomem",17),
    ("com_man",15),
    ("com_new",11),
    ("com_run",8),
    ("com_run_line",7),
    ("com_save",5),
    ("dollar",64),
    ("fcall_abs",49),
    ("fcall_ascp",60),
    ("fcall_lenp",59),
    ("fcall_pdl",50),
    ("fcall_peek",46),
    ("fcall_rnd",47),
    ("fcall_scrnp",61),
    ("fcall_sgn",48),
    ("op_aeq",22),
    ("op_and",29),
    ("op_aneq",23),
    ("op_div",21),
    ("op_eq_assign_int",113),
    ("op_eq_assign_str",112),
    ("op_eq_for",86),
    ("op_gtr",25),
    ("op_gtreq",24),
    ("op_hlin_at",107),
    ("op_less",28),
    ("op_lesseq",26),
    ("op_minus",19),
    ("op_mod",31),
    ("op_neq",27),
    ("op_not",55),
    ("op_or",30),
    ("op_plus",18),
    ("op_pow",32),
    ("op_seq",57),
    ("op_sneq",58),
    ("op_step",88),
    ("op_times",20),
    ("op_to",87),
    ("op_unary_minus",54),
    ("op_unary_plus",53),
    ("op_vlin_at",110),
    ("open_aexpr",56),
    ("open_dim_int",52),
    ("open_dim_str",34),
    ("open_fcall",63),
    ("open_int",45),
    ("open_slice",42),
    ("open_str",66),
    ("quote",40),
    ("sep_auto",14),
    ("sep_del",10),
    ("sep_dim_int",68),
    ("sep_dim_str",67),
    ("sep_hlin",106),
    ("sep_input_int",39),
    ("sep_input_str",38),
    ("sep_list",117),
    ("sep_next",90),
    ("sep_plot",104),
    ("sep_poke",101),
    ("sep_print_int",70),
    ("sep_print_null",71),
    ("sep_print_str",69),
    ("sep_scrn",62),
    ("sep_slice",35),
    ("sep_statement",3),
    ("sep_tab_int",73),
    ("sep_tab_null",74),
    ("sep_tab_str",72),
    ("sep_vlin",109),
    ("statement_call",77),
    ("statement_coloreq",102),
    ("statement_dim_int",79),
    ("statement_dim_str",78),
    ("statement_dsp_int",124),
    ("statement_dsp_str",123),
    ("statement_end",81),
    ("statement_for",85),
    ("statement_gosub",92),
    ("statement_goto",95),
    ("statement_gr",76),
    ("statement_hlin",105),
    ("statement_if",96),
    ("statement_inn",127),
    ("statement_input_int",84),
    ("statement_input_prompt",83),
    ("statement_input_str",82),
    ("statement_let",94),
    ("statement_list",118),
    ("statement_list_line",116),
    ("statement_next",89),
    ("statement_nodsp_int",121),
    ("statement_nodsp_str",120),
    ("statement_notrace",122),
    ("statement_plot",103),
    ("statement_poke",100),
    ("statement_pop",119),
    ("statement_print_int",98),
    ("statement_print_null",99),
    ("statement_print_str",97),
    ("statement_prn",126),
    ("statement_rem",93),
    ("statement_return",91),
    ("statement_tab",80),
    ("statement_text",75),
    ("statement_then",37),
    ("statement_then_line",36),
    ("statement_trace",125),
    ("statement_vlin",108),
    ("statement_vtab",111),
    ("unquote",41)
];

pub const DETOK_MAP: [(u8,&str);119] = [
    (3,":"),
    (4,"load"),
    (5,"save"),
    (6,"con"),
    (7,"run"),
    (8,"run"),
    (9,"del"),
    (10,","),
    (11,"new"),
    (12,"clr"),
    (13,"auto"),
    (14,","),
    (15,"man"),
    (16,"himem:"),
    (17,"lomem:"),
    (18,"+"),
    (19,"-"),
    (20,"*"),
    (21,"/"),
    (22,"="),
    (23,"#"),
    (24,">="),
    (25,">"),
    (26,"<="),
    (27,"<>"),
    (28,"<"),
    (29,"and"),
    (30,"or"),
    (31,"mod"),
    (32,"^"),
    (34,"("),
    (35,","),
    (36,"then"),
    (37,"then"),
    (38,","),
    (39,","),
    (40,"\""),
    (41,"\""),
    (42,"("),
    (45,"("),
    (46,"peek"),
    (47,"rnd"),
    (48,"sgn"),
    (49,"abs"),
    (50,"pdl"),
    (52,"("),
    (53,"+"),
    (54,"-"),
    (55,"not"),
    (56,"("),
    (57,"="),
    (58,"#"),
    (59,"len("),
    (60,"asc("),
    (61,"scrn("),
    (62,","),
    (63,"("),
    (64,"$"),
    (66,"("),
    (67,","),
    (68,","),
    (69,";"),
    (70,";"),
    (71,";"),
    (72,","),
    (73,","),
    (74,","),
    (75,"text"),
    (76,"gr"),
    (77,"call"),
    (78,"dim"),
    (79,"dim"),
    (80,"tab"),
    (81,"end"),
    (82,"input"),
    (83,"input"),
    (84,"input"),
    (85,"for"),
    (86,"="),
    (87,"to"),
    (88,"step"),
    (89,"next"),
    (90,","),
    (91,"return"),
    (92,"gosub"),
    (93,"rem"),
    (94,"let"),
    (95,"goto"),
    (96,"if"),
    (97,"print"),
    (98,"print"),
    (99,"print"),
    (100,"poke"),
    (101,","),
    (102,"color="),
    (103,"plot"),
    (104,","),
    (105,"hlin"),
    (106,","),
    (107,"at"),
    (108,"vlin"),
    (109,","),
    (110,"at"),
    (111,"vtab"),
    (112,"="),
    (113,"="),
    (114,")"),
    (116,"list"),
    (117,","),
    (118,"list"),
    (119,"pop"),
    (120,"nodsp"),
    (121,"nodsp"),
    (122,"notrace"),
    (123,"dsp"),
    (124,"dsp"),
    (125,"trace"),
    (126,"pr#"),
    (127,"in#")
];
