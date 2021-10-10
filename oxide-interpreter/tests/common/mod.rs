use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, process};

use oxide_interpreter::{Engine, StdStreamProvider};

pub fn compare_output(sample_file: &str, output_file: &str) {
    let stdout = Rc::new(RefCell::new(Vec::<u8>::new()));
    let stderr = Rc::new(RefCell::new(Vec::<u8>::new()));

    let vecout = Rc::clone(&stdout);
    let _vecerr = Rc::clone(&stderr);

    let stream_provider = StdStreamProvider::new(Some((Some(stdout), Some(stderr), None)));

    let script = fs::read_to_string(sample_file).expect("Error while reading file.");

    let engine = Engine::new(|errs| {
        for err in errs {
            eprintln!("{}", err);
        }
        process::exit(1);
    });

    let ast = engine.ast(script);
    let _val = engine.run(&ast, &[], Some(Box::new(stream_provider)));

    let actual = &*vecout.borrow();
    let actual = String::from_utf8_lossy(actual);

    let expected = fs::read_to_string(output_file).expect("Error while reading file.");

    assert_eq!(actual, expected);
}
