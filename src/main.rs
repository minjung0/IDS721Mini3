use chrono::prelude::*;
use chrono_tz::Tz;
use chrono_tz::US::Eastern;
use chrono_tz::US::Pacific;
use chrono_tz::Asia::Seoul;
use chrono_tz::Europe::London;

fn main() {

let est: DateTime<Tz> = Utc::now().with_timezone(&Eastern);
let pst: DateTime<Tz> = Utc::now().with_timezone(&Pacific);
let seoul: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
let london: DateTime<Tz> = Utc::now().with_timezone(&London);

println!("Estern Time: {}", est);
println!("Pacific Time: {}", pst);
println!("Seoul Time: {}", seoul);
println!("London Time: {}", london);
}
