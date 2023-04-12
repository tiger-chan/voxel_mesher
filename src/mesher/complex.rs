/// # Voxel Meshing Strategy
///
/// This crate provides a voxel meshing strategy for representing complex shapes within a voxel space. Each voxel in this implementation can contain N number of quads per face, and includes a culling feature to remove invisible faces.
///
/// ## Quads per Face
///
/// In this voxel meshing strategy, each voxel can contain N number of quads per face. Quads are four-sided polygons commonly used in computer graphics.
///
/// ## Culling
///
/// One important feature of this voxel meshing strategy is the ability to indicate whether a face should be culled in the inverse direction. Culling is the process of removing faces that are not visible to the viewer, in order to improve performance. When a face is culled, it is not rendered and is therefore invisible to the viewer.
///
/// If a voxel indicates that culling will occur, then everything behind that point will be culled. For example, if you are looking at a voxel from the positive Z direction and the voxel indicates that culling will occur, then everything behind the voxel in the negative Z direction will be culled.
///
#[allow(dead_code)]
struct ComplexMesher;