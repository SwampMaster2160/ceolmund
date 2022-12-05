/// If and when an entity can move to a tile.
pub enum TileMovementType {
	Clear, // Can always move to tile.
	Wall, // Can never move to tile.
}