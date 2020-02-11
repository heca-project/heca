pub mod constants;
use crate::algorithms::torah_reading::constants::chol_to_json_string;
use crate::algorithms::torah_reading::constants::parsha_to_json_string;
use crate::algorithms::torah_reading::constants::special_parsha_to_json_string;
use crate::algorithms::torah_reading::constants::yt_to_json_string;
use crate::args::types::DayVal;
use crate::prelude::Json;
use heca_lib::prelude::TorahReading;

pub(crate) fn json_print(tr: &TorahReading, dv: &DayVal, json: &mut Json<'_, '_>) {
    match tr {
        TorahReading::YomTov(yt) => {
            json.print_map_unchecked("type", "YomTov");
            json.next();
            json.print_map_unchecked("name", yt_to_json_string(&yt));
            if let Some(candle_lighting_time) = dv.candle_lighting {
                match candle_lighting_time {
                    Some(t) => {
                        json.next();
                        json.print_map_unchecked("candleLighting", &t.to_rfc3339())
                    }
                    None => {
                        json.next();
                        json.print_map_unchecked("candleLighting", "undefined")
                    }
                };
            }
        }
        TorahReading::Chol(chol) => {
            json.print_map_unchecked("type", "Chol");
            json.next();
            json.print_map_unchecked("name", chol_to_json_string(&chol));
        }
        TorahReading::Shabbos(parsha) => {
            json.print_map_unchecked("type", "Shabbos");
            json.next();
            json.print_map_unchecked("name", parsha_to_json_string(&parsha));

            if let Some(candle_lighting_time) = dv.candle_lighting {
                match candle_lighting_time {
                    Some(t) => {
                        json.next();
                        json.print_map_unchecked("candleLighting", &t.to_rfc3339())
                    }
                    None => {
                        json.next();
                        json.print_map_unchecked("candleLighting", "undefined")
                    }
                };
            }
        }
        TorahReading::SpecialParsha(special_parsha) => {
            json.print_map_unchecked("type", "SpecialParsha");
            json.next();
            json.print_map_unchecked("name", special_parsha_to_json_string(&special_parsha));
        }
    };
}
