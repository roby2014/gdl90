//! GDL90 Height Above Terrain message.
//!
//! | Byte # | Name                 | Size  | Value                                         |
//! |--------|----------------------|-------|-----------------------------------------------|
//! | 1      |Message ID            | 1     | 9                                             |
//! | 2-3    |Height Above Terrain  | 2     | Height Above Terrain. Resolution = 1 foot     |
//! |        |Total length          | 3     |                                                |
//!

use binrw::BinRead;

#[derive(BinRead, Debug)]
#[br(little)]
pub struct HeigthAboveTerrain {
    pub hat: u16, // TODO: custom type
}
