// Copyright 2017 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

use chrono::{NaiveDate, NaiveDateTime};
use log::info;
use std::path::PathBuf;
use structopt;
use structopt::StructOpt;
use transit_model;
use transit_model::Model;
use transit_model::Result;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "restrict-validity-period",
    about = "Restrict the validity period and purges out of date data.",
    rename_all = "kebab-case"
)]
struct Opt {
    /// input directory.
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    input: PathBuf,

    /// start of the validity period
    #[structopt(short, long)]
    start_validity_date: NaiveDate,

    /// end of the validity period
    #[structopt(short, long)]
    end_validity_date: NaiveDate,

    /// output directory
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// current datetime
    #[structopt(
        short = "x",
        long,
        parse(try_from_str),
        default_value = &transit_model::CURRENT_DATETIME
    )]
    current_datetime: NaiveDateTime,
}

fn run(opt: Opt) -> Result<()> {
    info!("Launching restrict-validity-period...");

    let model = transit_model::ntfs::read(opt.input)?;
    let mut collections = model.into_collections();
    collections.restrict_period(opt.start_validity_date, opt.end_validity_date)?;
    collections.sanitize()?;
    let model = Model::new(collections)?;
    transit_model::ntfs::write(&model, opt.output, opt.current_datetime)?;
    Ok(())
}

fn main() {
    tartare_tools::runner::launch_run(run);
}
