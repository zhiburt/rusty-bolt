//////////////// LOGGING ///////////////////

//use std::io::{stderr, Write};
//
//#[macro_use]
//extern crate log;
//use log::{LogRecord, LogLevel, LogMetadata};
//
//struct SimpleLogger;
//
//impl log::Log for SimpleLogger {
//    fn enabled(&self, metadata: &LogMetadata) -> bool {
//        metadata.level() <= LogLevel::Debug
//    }
//
//    fn log(&self, record: &LogRecord) {
//        if self.enabled(record.metadata()) {
//            let _ = writeln!(stderr(), "[{}]  {}", record.level(), record.args());
//        }
//    }
//}

//////////////////////////////////////////////////////////////////////

use std::env;
use std::collections::{VecDeque, HashMap};

#[macro_use]
extern crate cypherstream;
use cypherstream::{CypherStream};

#[macro_use]
extern crate packstream;
use packstream::value::{Value, Data};

fn main() {
    let mut args = env::args();

    let statement = match args.nth(1) {
        Some(string) => string,
        _ => String::from("RETURN $x"),
    };
    let parameters = parameters!("x" => 1);

//    let _ = log::set_logger(|max_log_level| {
//        max_log_level.set(log::LogLevelFilter::Debug);
//        Box::new(SimpleLogger)
//    });

    let cypher = CypherStream::connect("[::1]:7687", "neo4j", "password").unwrap();
    dump(cypher, &statement[..], parameters);

}

fn dump(mut cypher: CypherStream, statement: &str, parameters: HashMap<&str, Value>) {
    // begin transaction
//    cypher.begin_transaction(None);

    // execute statement
    let result = cypher.run(statement, parameters);
    println!("{}", result.keys());

    // iterate result
    let mut counter: usize = 0;
    let mut records: VecDeque<Data> = VecDeque::new();
    while cypher.fetch(&result, &mut records) > 0 {
        for record in records.drain(..) {
            println!("{}", record);
            counter += 1;
        }
    }
    let _ = cypher.fetch_summary(&result);
    println!("({} record{})", counter, match counter { 1 => "", _ => "s" });

    // commit transaction
//    cypher.commit_transaction();

}
