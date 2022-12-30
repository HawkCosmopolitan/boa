use std::{error::Error, fs::File};

use boa_icu_provider::data_root;
use icu_datagen::{all_keys, datagen, CldrLocaleSubset, Out, SourceData};

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let source_data = SourceData::default()
        .with_cldr_latest(CldrLocaleSubset::Modern)?
        .with_icuexport_latest()?;

    let blob_out = Out::Blob(Box::new(File::create(
        data_root().join("icudata.postcard"),
    )?));

    datagen(None, &all_keys(), &source_data, [blob_out].into()).map_err(Into::into)
}