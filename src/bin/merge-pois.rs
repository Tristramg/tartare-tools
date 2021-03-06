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

use log::info;
use std::path::PathBuf;
use structopt::StructOpt;
use tartare_tools::{poi::merge::merge, Result};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "merge-pois",
    about = "Merge NAvitia POI files.",
    rename_all = "kebab-case"
)]
struct Opt {
    /// Navitia POI files
    #[structopt(name = "INPUTS", required = true, min_values = 2, parse(from_os_str))]
    pois: Vec<PathBuf>,

    /// Output poi file.
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn run(opt: Opt) -> Result<()> {
    info!("Launching merge-pois.");
    let model = merge(&mut opt.pois.into_iter())?;
    model.save_to_path(opt.output)
}

fn main() {
    tartare_tools::runner::launch_run(run);
}
