/// * The binary format is as follows:
/// *   - Version (u32) (not included in count, not included in deser, included in serialization)
/// *   - Width (u32)
/// *   - Height (u32)
/// *   - CellSize (u32)
/// *   - GridEnabled (u32)
/// *   - AliveColor ([u32, u32, u32, u32])
/// *   - DeadColor ([u32, u32, u32, u32])
/// *   - GridColor ([u32, u32, u32, u32])
/// *   - WrappingEnabled (u32)
export function fmtLengthV0() {
    return 4 * 17;
}

/**
 * Serializes the given object of attributes into a binary format.
 *
 * @param {Object} attrs - Object containing the attributes to serialize.
 *     Must have the following properties:
 *       - width: {number} Width of the game board.
 *       - height: {number} Height of the game board.
 *       - cellSize: {number} Size of each cell in the game board.
 *       - gridEnabled: {boolean} Whether the grid is enabled.
 *       - aliveColor: {number[]} RGBA color of alive cells.
 *       - deadColor: {number[]} RGBA color of dead cells.
 *       - gridColor: {number[]} RGB color of the grid.
 *       - wrappingEnabled: {boolean} Whether the map is wrapped.
 * @returns {ArrayBuffer} Serialized binary representation of the attributes.
 */
export function serializeAttributesV0(attrs) {
    const attrArray = [
        0, // version number included here
        attrs.width,
        attrs.height,
        attrs.cellSize,
        attrs.gridEnabled,
        attrs.aliveColor[0],
        attrs.aliveColor[1],
        attrs.aliveColor[2],
        attrs.aliveColor[3],
        attrs.deadColor[0],
        attrs.deadColor[1],
        attrs.deadColor[2],
        attrs.deadColor[3],
        attrs.gridColor[0],
        attrs.gridColor[1],
        attrs.gridColor[2],
        attrs.gridColor[3],
        attrs.wrappingEnabled,
    ];
    const buffer = new ArrayBuffer(attrArray.length * 4);
    const view = new DataView(buffer);
    attrArray.forEach((attr, index) => {
        view.setUint32(index * 4, attr, true);
    });
    return buffer;
}

/**
 * Deserializes the given binary representation of attributes into an object.
 *
 * @param {ArrayBuffer} buffer - Binary representation of the attributes.
 * @returns {Object} Deserialized object with the following properties:
 *     - width: {number} Width of the game board.
 *     - height: {number} Height of the game board.
 *     - cellSize: {number} Size of each cell in the game board.
 *     - gridEnabled: {boolean} Whether the grid is enabled.
 *     - aliveColor: {number[]} RGBA color of alive cells.
 *     - deadColor: {number[]} RGBA color of dead cells.
 *     - gridColor: {number[]} RGB color of the grid.
 *     - wrappingEnabled: {boolean} Whether the map is wrapped.
 */
export function deserializeAttributesV0(buffer) {
    const view = new DataView(buffer);
    const attributes = {
        width: view.getUint32(0, true),
        height: view.getUint32(4, true),
        cellSize: view.getUint32(8, true),
        gridEnabled: view.getUint32(12, true),
        aliveColor: [view.getUint32(16, true), view.getUint32(20, true), view.getUint32(24, true), view.getUint32(28, true)],
        deadColor: [view.getUint32(32, true), view.getUint32(36, true), view.getUint32(40, true), view.getUint32(44, true)],
        gridColor: [view.getUint32(48, true), view.getUint32(52, true), view.getUint32(56, true), view.getUint32(60, true)],
        wrappingEnabled: view.getUint32(64, true),
    };
    return attributes;
}