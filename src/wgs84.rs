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

use crate::{objects::Coord, Result};

/// Interface dealing with spatial coordinates in EPSG:4326 format aka WGS84
/// See https://epsg.io/4326
pub trait WGS84Coordinates {
    /// Returns a WGS84 spatial coordinates
    fn coord(&self) -> Result<Coord>;
}
