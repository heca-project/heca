use heca_lib::HebrewDate;
use heca_lib::holidays::ScheduleLocation;
use chrono::prelude::*;
struct MainArgs{
    config: String,
    output_type: OutputType,
    language: Language,
    command: Command
}
enum Language {
    English,
    Hebrew,
}

enum Command {
    Convert(ConvertArgs),
    List(ListArgs),
}
enum OutputType {
    Regular,
    Pretty,
    JSON,
}
struct ConvertArgs {
   date: ConvertType,
}

enum ConvertType{
    Gregorian(chrono::DateTime<Utc>),
    Hebrew(HebrewDate),
}

struct ListArgs{
    year: YearType,
    location: ScheduleLocation,
    events: Vec<Events>,
    shuffle: bool,
}
enum YearType {
    Gregorian(u64),
    Hebrew(u64),
}

enum Events {
    YomTov,
    Parsha,
    SpecialParshas,
    Holidays,
}

