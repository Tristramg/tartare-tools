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

use std::collections::HashMap;
use std::path::Path;
use tartare_tools::improve_stop_positions;
use transit_model::ntfs;
use transit_model::test_utils::*;

#[test]
fn test_map_no_force() {
    test_in_tmp_dir(|path| {
        let mut ntfs_network_to_osm = HashMap::new();
        ntfs_network_to_osm.insert("RTM", "RTM");
        let input_dir = "./tests/fixtures/map-ntfs-with-osm/input";
        let model = ntfs::read(input_dir).unwrap();
        let enriched_model = improve_stop_positions::enrich_object_codes(
            Path::new("./tests/fixtures/map-ntfs-with-osm/marseille-lite.osm.pbf"),
            model,
            ntfs_network_to_osm,
            false,
        )
        .unwrap();
        transit_model::ntfs::write(&enriched_model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(
            &path,
            Some(vec!["object_codes.txt"]),
            "./tests/fixtures/map-ntfs-with-osm/output/no_force",
        );
    });
}

#[test]
fn test_map_force() {
    test_in_tmp_dir(|path| {
        let mut ntfs_network_to_osm = HashMap::new();
        ntfs_network_to_osm.insert("RTM", "RTM");
        let input_dir = "./tests/fixtures/map-ntfs-with-osm/input";
        let model = ntfs::read(input_dir).unwrap();
        let enriched_model = improve_stop_positions::enrich_object_codes(
            Path::new("./tests/fixtures/map-ntfs-with-osm/marseille-lite.osm.pbf"),
            model,
            ntfs_network_to_osm,
            true,
        )
        .unwrap();
        transit_model::ntfs::write(&enriched_model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(
            &path,
            Some(vec!["object_codes.txt"]),
            "./tests/fixtures/map-ntfs-with-osm/output/force",
        );
    });
}
