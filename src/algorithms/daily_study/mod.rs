pub mod daf_yomi;
pub mod rambam;
pub mod yerushalmi;
use crate::prelude::Json;
use crate::prelude::JsonPrinter;
use daf_yomi::*;
use rambam::*;
use yerushalmi::*;
#[derive(Debug, Clone, PartialEq)]
pub enum DailyStudy {
    DafYomi,
    Rambam(RambamChapters),
    YerushalmiYomi,
}

#[derive(Debug, Clone)]
pub enum DailyStudyOutput {
    Daf(Daf),
    RambamThreeChapters(RambamThreeChapter),
    RambamOneChapters(RambamChapter),
    YerushalmiYomi(YerushalmiYomi),
}

impl JsonPrinter for DailyStudyOutput {
    fn json_print(&self, json: &mut Json<'_, '_>) {
        match self {
            DailyStudyOutput::Daf(daf) => daf.json_print(json),
            DailyStudyOutput::RambamThreeChapters(halacha) => halacha.json_print(json),
            DailyStudyOutput::RambamOneChapters(halacha) => halacha.json_print(json),
            DailyStudyOutput::YerushalmiYomi(yerushalmi_yomi) => yerushalmi_yomi.json_print(json),
        };
    }
}
