use csgrs::traits::CSG;

type Mesh = csgrs::mesh::Mesh<()>;

fn main() {
    // Create a cube
    let cube: Mesh = Mesh::cube(2.0, None); // 2×2×2 cube at origin, no metadata

    // Create sphere at (1, 1, 1) with radius 1.25:
    let sphere: Mesh = Mesh::sphere(1.25, 16, 8, None).translate(1.0, 1.0, 1.0);

    // Perform a difference operation:
    let result = cube.difference(&sphere);

    // Write the result as an ASCII STL:
    let stl = result.to_stl_ascii("cube_minus_sphere");
    std::fs::write("cube_sphere_difference.stl", stl).unwrap();
}
