use super::square::Location;

#[derive(Debug)]
pub enum MovePieceError {
    NoSrcPiece(Location),
    SrcDstPiecesSideSame(Location, Location)
}

