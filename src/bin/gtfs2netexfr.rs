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

use chrono::NaiveDateTime;
use failure::bail;
use log::info;
use std::path::PathBuf;
use structopt;
use structopt::StructOpt;
use transit_model;
use transit_model::Result;

#[derive(Debug, StructOpt)]
#[structopt(name = "gtfs2netexfr", about = "Convert a GTFS to NeTEx France.")]
struct Opt {
    /// input directory.
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    input: PathBuf,

    /// output directory
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// config file
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>,

    /// OnDemandTransport GTFS source
    #[structopt(short = "t", long = "on-demand-transport")]
    odt: bool,

    /// name for the participant
    #[structopt(short, long)]
    participant: String,

    /// code for the provider of stops
    #[structopt(short, long)]
    stop_provider: Option<String>,

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
    info!("Launching gtfs2netexfr...");

    let model = if opt.input.is_file() {
        transit_model::gtfs::read_from_zip(opt.input, opt.config, None, opt.odt)?
    } else if opt.input.is_dir() {
        transit_model::gtfs::read_from_path(opt.input, opt.config, None, opt.odt)?
    } else {
        bail!("Invalid input data: must be an existing directory or a ZIP archive");
    };

    let netex_exporter = transit_model::netex_france::Exporter::new(
        &model,
        opt.participant,
        opt.stop_provider,
        opt.current_datetime,
    );
    netex_exporter.write(opt.output)?;
    Ok(())
}

fn main() {
    tartare_tools::runner::launch_run(run);
}
