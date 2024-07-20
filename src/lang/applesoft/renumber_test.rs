use super::renumber::Renumberer;

fn test_renumber(test_code: &str,expected: &str,beg: usize,end: usize,first: usize,step: usize,should_fail: bool) {
	let mut renumberer = Renumberer::new();
	let result = renumberer.renumber(test_code, beg, end, first, step);
	match should_fail {
		true => if let Ok(_actual) = result {
			// if renumbering should fail, but instead works, the test has failed
			assert!(false);
		},
		false => if let Ok(actual) = renumberer.renumber(test_code, beg, end, first, step) {
			assert_eq!(actual,String::from(expected));
		}
	}
}

fn test_move(test_code: &str,expected: &str,beg: usize,end: usize,first: usize,step: usize,should_fail: bool) {
	let mut renumberer = Renumberer::new();
	renumberer.set_flags(1);
	let result = renumberer.renumber(test_code, beg, end, first, step);
	match should_fail {
		true => if let Ok(_actual) = result {
			// if renumbering should fail, but instead works, the test has failed
			assert!(false);
		},
		false => if let Ok(actual) = renumberer.renumber(test_code, beg, end, first, step) {
			assert_eq!(actual,String::from(expected) + "\n");
		}
	}
}

mod valid_cases {
    #[test]
	fn zero_start() {
		let test_code = "0 HOME\n20 PRINT X\n30 END";
		let expected = "100 HOME\n101 PRINT X\n102 END";
		super::test_renumber(test_code, expected,0,usize::MAX,100,1,false);
	}
    #[test]
	fn largest_num() {
		let test_code = "0 HOME\n20 PRINT X\n30 END";
		let expected = "63993 HOME\n63996 PRINT X\n63999 END";
		super::test_renumber(test_code, expected,0,usize::MAX,63993,3,false);
	}
    #[test]
	fn segment() {
		let test_code = "10 HOME\n20 INPUT X\n30 PRINT X\n40 END";
		let expected = "10 HOME\n27 INPUT X\n29 PRINT X\n40 END";
		super::test_renumber(test_code, expected,20,40,27,2,false);
	}
}

mod invalid_cases {
    #[test]
	fn breaks_lower_bound() {
		let test_code = "10 HOME\n20 PRINT X\n30 END";
		let expected = "";
		super::test_renumber(test_code, expected,20,usize::MAX,9,1,true);
	}
    #[test]
	fn breaks_upper_bound() {
		let test_code = "10 HOME\n20 PRINT X\n30 END";
		let expected = "";
		super::test_renumber(test_code, expected,0,30,25,5,true);
	}
    #[test]
	fn breaks_max() {
		let test_code = "10 HOME\n20 PRINT X\n30 END";
		let expected = "";
		super::test_renumber(test_code, expected,0,usize::MAX,63800,100,true);
	}
    #[test]
	fn move_not_allowed() {
		let test_code = "0 HOME\n20 PRINT X\n30 END";
		let expected = "30 END\n40 HOME\n50 PRINT X";
		super::test_renumber(test_code, expected,0,30,40,10,true);
	}
}

mod valid_moves {
    #[test]
	fn with_on_x_goto() {
		let test_code = "0 HOME\n20 GOTO 30\n30 END\n40 ON X GOTO 0,20,30";
		let expected = "40 ON X GOTO 100,101,102\n100 HOME\n101 GOTO 102\n102 END";
		super::test_move(test_code, expected,0,40,100,1,false);
	}
    #[test]
	fn simple_move() {
		let test_code = "0 HOME\n20 PRINT X\n30 END";
		let expected = "30 END\n40 HOME\n50 PRINT X";
		super::test_move(test_code, expected,0,30,40,10,false);
	}
    #[test]
	fn inner_move() {
		let test_code = "10 HOME\n20 INPUT X\n30 PRINT X\n40 END";
		let expected = "10 HOME\n40 END\n1000 INPUT X\n1002 PRINT X";
		super::test_move(test_code, expected,20,40,1000,2,false);
	}
}