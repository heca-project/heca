use crate::prelude::Json;
use crate::prelude::JsonPrinter;
use core::num::NonZeroI8;
use heca_lib::prelude::HebrewMonth;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomHoliday {
    pub printable: String,
    pub json: String,
    pub date: DayMonth,
    pub if_not_exists: Option<Vec<DayMonth>>,
}

impl JsonPrinter for CustomHoliday {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        json.print_map_unchecked("type", "CustomHoliday");
        json.next();
        json.print_map_unchecked("name", &self.json);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayMonth {
    pub month: HebrewMonth,
    pub day: NonZeroI8,
}
