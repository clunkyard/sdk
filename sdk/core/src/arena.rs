//! The arena: its size, gravity and ground.

use crate::generated::*;
use crate::maths::{floor, max, min};
use crate::message::Terrain;
use crate::runtime::Robot;

pub fn size(r: &Robot) -> Result<Vec3, Error> {
    let hello = r.hello()?;
    Ok(Vec3 { x: hello.width, y: hello.height, z: hello.depth })
}

pub fn gravity(r: &Robot) -> Result<Vec3, Error> {
    Ok(r.hello()?.gravity)
}

pub fn terrain_kind(r: &Robot) -> Result<TerrainKind, Error> {
    Ok(r.hello()?.terrain_kind)
}

/// The ground under (x, y): 0 with no ground, groundZ when flat, and on
/// hills the bilinear interpolation of the cols × rows grid, which spans the
/// arena from (0, 0) to (width, height); x and y are first clamped into it.
pub fn ground_height(r: &Robot, x: f64, y: f64) -> Result<f64, Error> {
    let hello = r.hello()?;
    let (cols, rows, heights) = match &hello.terrain {
        Terrain::None => return Ok(0.0),
        Terrain::Flat { ground_z } => return Ok(*ground_z),
        Terrain::Hills { cols, rows, heights } => (*cols, *rows, heights),
    };
    let (width, height) = (hello.width, hello.height);
    let u = min(width, max(0.0, x)) / width * (cols - 1.0);
    let v = min(height, max(0.0, y)) / height * (rows - 1.0);
    let c0 = min(cols - 2.0, floor(u));
    let r0 = min(rows - 2.0, floor(v));
    let (fu, fv) = (u - c0, v - r0);
    let at = |c: f64, r: f64| {
        let index = r * cols + c;
        if index >= 0.0 { heights.get(index as usize).copied().unwrap_or(f64::NAN) } else { f64::NAN }
    };
    let low = at(c0, r0) + (at(c0 + 1.0, r0) - at(c0, r0)) * fu;
    let high = at(c0, r0 + 1.0) + (at(c0 + 1.0, r0 + 1.0) - at(c0, r0 + 1.0)) * fu;
    Ok(low + (high - low) * fv)
}
