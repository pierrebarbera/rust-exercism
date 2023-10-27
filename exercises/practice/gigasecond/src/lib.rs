use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    let result = start + time::Duration::SECOND * 1e9;
    println!("start: {start}");
    println!("after gigaseconds: {result} \n");
    return result;

}
