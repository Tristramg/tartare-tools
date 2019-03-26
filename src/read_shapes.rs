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

use crate::Result;
use failure::bail;
use failure::format_err;
use log::warn;
use navitia_model::collection::CollectionWithId;
use navitia_model::collection::Id as NtfsId;
use navitia_model::model::Collections;
use navitia_model::objects::{Codes, Geometry, Line as NtfsLine, Route as NtfsRoute};
use osm_transit_extractor::*;
use std::collections::HashMap;
use std::path::Path;

macro_rules! skip_fail {
    ($res:expr) => {{
        use log::warn;
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        }
    }};
}

pub trait WithGeometry {
    fn set_geometry_id(&mut self, geometry_id: String) -> Result<()>;
}

impl WithGeometry for NtfsLine {
    fn set_geometry_id(&mut self, geometry_id: String) -> Result<()> {
        if self.geometry_id.is_some() {
            bail!("geometry already exists for line {:?}", self.id);
        } else {
            self.geometry_id = Some(geometry_id);
            return Ok(());
        }
    }
}

impl WithGeometry for NtfsRoute {
    fn set_geometry_id(&mut self, geometry_id: String) -> Result<()> {
        if self.geometry_id.is_some() {
            bail!("geometry already exists for route {:?}", self.id);
        } else {
            self.geometry_id = Some(geometry_id);
            return Ok(());
        }
    }
}

fn osm_objects_by_id<'a, T>(
    osm_objects: &'a Option<Vec<T>>,
    object_type: &str,
) -> Result<HashMap<String, &'a T>>
where
    T: Id<T>,
{
    match osm_objects {
        Some(objects) => Ok(objects.iter().map(|o| (o.id().to_string(), o)).collect()),
        None => bail!("no {} found in osm", object_type),
    }
}

fn populate_shapes_for_collection<N, O>(
    geometries: &mut Vec<Geometry>,
    ntfs_objects: &mut CollectionWithId<N>,
    osm_objects: &HashMap<String, &O>,
    object_type: &str,
) -> Result<CollectionWithId<N>>
where
    O: Shape + Id<O>,
    N: Codes + NtfsId<N> + WithGeometry,
{
    let mut objects = ntfs_objects.take();
    let mut generated_geo_obj_id = 0;
    for obj in objects.iter_mut() {
        if let Some((_, osm_obj_id)) = obj
            .codes()
            .iter()
            .find(|(key, _)| *key == format!("osm_{}_id", object_type))
        {
            generated_geo_obj_id = generated_geo_obj_id + 1;
            match osm_objects.get(osm_obj_id) {
                Some(osm_object) => {
                    if osm_object.get_shape().is_empty() {
                        warn!(
                            "no geometry found in osm for {:?} <-> {:?}",
                            obj.id(),
                            osm_object.id()
                        );
                        continue;
                    }
                    let geo_id = format!("geo:{}:osm:{}", object_type, generated_geo_obj_id);
                    skip_fail!(obj.set_geometry_id(geo_id.clone()));
                    geometries.push(Geometry {
                        id: geo_id,
                        geometry: shape_to_multi_line_string(*osm_object).into(),
                    });
                }
                None => bail!("relation {} not found in osm", &osm_obj_id),
            }
        }
    }
    CollectionWithId::new(objects)
}

pub fn from_osm(osm_pbf_path: &Path, collections: &mut Collections) -> Result<()> {
    let mut parsed_pbf = parse_osm_pbf(
        osm_pbf_path
            .to_str()
            .ok_or_else(|| format_err!("osm pbf path is not valid"))?,
    );
    let objects = get_osm_tcobjects(&mut parsed_pbf, false);
    let osm_lines_by_id = osm_objects_by_id(&objects.lines, "lines")?;
    let osm_routes_by_id = osm_objects_by_id(&objects.routes, "routes")?;
    let mut geometries = collections.geometries.take();
    collections.lines = populate_shapes_for_collection(
        &mut geometries,
        &mut collections.lines,
        &osm_lines_by_id,
        "line",
    )?;
    collections.routes = populate_shapes_for_collection(
        &mut geometries,
        &mut collections.routes,
        &osm_routes_by_id,
        "route",
    )?;
    collections.geometries = CollectionWithId::new(geometries)?;

    Ok(())
}
