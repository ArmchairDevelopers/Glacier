use bytes::{Buf, BytesMut};
use glacier_resource::terrain::streaming_tree::{RasterTreeNode, TerrainStreamingTree};
use glam::Vec3A;
use image::{ImageBuffer, Luma};
use std::io::{BufWriter, Write};

/// Perform bilinear interpolation on Y-axis based on four corners and x, z coordinates.
/// The function normalizes x and z inputs relative to the corners.
fn bilinear_interpolate_y(
    top_left: Vec3A,
    top_right: Vec3A,
    bottom_left: Vec3A,
    bottom_right: Vec3A,
    x: f32,
    z: f32,
) -> f32 {
    const EPSILON: f32 = 1e-6;

    // Find min and max for X and Z based on corner points
    let min_x = top_left.x.min(bottom_left.x);
    let max_x = top_right.x.max(bottom_right.x);
    let min_z = top_left.z.min(top_right.z);
    let max_z = bottom_left.z.max(bottom_right.z);

    //println!("min_x: {}, max_x: {}, min_z: {}, max_z: {}", min_x, max_x, min_z, max_z);

    // Normalize x and z within the bounds of the corners with boundary adjustment
    let normalized_x = if (max_x - min_x).abs() < EPSILON {
        0.5 // Default to midpoint if range is too small
    } else {
        let norm_x = (x - min_x) / (max_x - min_x);
        // Adjust values very close to 0 or 1
        if norm_x <= EPSILON {
            0.01
        } else if norm_x >= 1.0 - EPSILON {
            0.99
        } else {
            norm_x
        }
    };

    let normalized_z = if (max_z - min_z).abs() < EPSILON {
        0.5 // Default to midpoint if range is too small
    } else {
        let norm_z = (z - min_z) / (max_z - min_z);
        // Adjust values very close to 0 or 1
        if norm_z <= EPSILON {
            0.01
        } else if norm_z >= 1.0 - EPSILON {
            0.99
        } else {
            norm_z
        }
    };

    //println!("Normalized X: {}, Normalized Z: {}", normalized_x, normalized_z);

    // Interpolate along the X-axis on the top and bottom edges
    let top_y = top_left.y + (top_right.y - top_left.y) * normalized_x;
    let bottom_y = bottom_left.y + (bottom_right.y - bottom_left.y) * normalized_x;

    // Interpolate along the Z-axis between the top and bottom interpolated Y values
    top_y + (bottom_y - top_y) * normalized_z
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = include_bytes!(
        "../../glacier_resource/tests/data/naboo_01_terrain.streamingtree_win32.res"
    );
    let mut buf = BytesMut::from(&data[..]);
    buf.advance(16);

    let tree = TerrainStreamingTree::load(&mut buf);

    let heightfield = tree
        .nodes()
        .iter()
        .find_map(|node| {
            if let RasterTreeNode::HeightfieldTree(heightfield) = node {
                Some(heightfield)
            } else {
                None
            }
        })
        .expect("HeightfieldNode not found");

    //println!("{:?}", heightfield);

    // let mut vertices = Vec::new();
    // let mut indices = Vec::new();

    // for node in &heightfield.nodes {
    //     if !node.has_data() || node.children_have_data {
    //         continue;
    //     }

    //     let coords = node.samples_to_world_coords();
    //     let base_index = vertices.len() as u32;

    //     for coord in coords {
    //         vertices.push((coord.x, coord.y, coord.z));
    //     }

    //     let node_grid_size = node.node_samples_per_side;
    //     for y in 0..(node_grid_size - 1) {
    //         for x in 0..(node_grid_size - 1) {
    //             let top_left = base_index + (y * node_grid_size + x) as u32;
    //             let top_right = top_left + 1;
    //             let bottom_left = top_left + node_grid_size as u32;
    //             let bottom_right = bottom_left + 1;

    //             indices.push([top_left, bottom_left, top_right]);
    //             indices.push([top_right, bottom_left, bottom_right]);
    //         }
    //     }
    // }

    let mut img = ImageBuffer::new(8193, 8193);

    let mut min_height = 0;
    let mut max_height = 4096;

    for node in &heightfield.nodes {
        if !node.has_data() || node.children_have_data {
            continue;
        }

        // Retrieve world coordinates and heights
        let coords = node.samples_to_world_coords();

        // Update min and max height based on each sample's height
        for coord in coords {
            // if coord.y < min_height {
            //     //min_height = coord.y;
            // }
            // if coord.y > max_height {
            //     //max_height = coord.y;
            // }
        }
    }

    // Define world-to-image scaling factors
    let scale_x = 8192.0 / 8192.0; // Assuming world bounds -4096 to 4096 maps to image bounds 0 to 8192
    let scale_z = 8192.0 / 8192.0;

    for img_y in 0..8192 {
        for img_x in 0..8192 {
            // Map image coordinates to world coordinates
            let world_x = (img_x as f32 / scale_x) - 4096.0;
            let world_z = (img_y as f32 / scale_z) - 4096.0;

            let node = heightfield.get_best_node(world_x, world_z);

            // Find the four closest samples in the heightfield
            let (top_left, top_right, bottom_left, bottom_right) =
                node.find_closest_samples(world_x, world_z);

            //println!("{:?} {:?} {:?} {:?}", world_x, world_z, virtual_x, virtual_z);

            // Perform bilinear interpolation on the height values
            let interpolated_height = bilinear_interpolate_y(
                top_left,
                top_right,
                bottom_left,
                bottom_right,
                world_x,
                world_z,
            );

            //println!("{:?} {}", node.bounding_box(), interpolated_height);

            // Normalize the interpolated height to grayscale (0-255)
            let gray_value = ((interpolated_height - min_height) / (max_height - min_height)
                * 255.0)
                .clamp(0.0, 255.0) as u8;

            // Set the pixel
            img.put_pixel(img_x, img_y, Luma([gray_value]));
        }

        if img_y % 100 == 0 {
            println!("Row {}", img_y);
        }
    }

    println!("Writing image");

    // Save the image as PNG
    img.save("heightmap.png").expect("Failed to save image");

    // let mut obj_file = BufWriter::new(std::fs::File::create("terrain.obj")?);
    // for vertex in &vertices {
    //     writeln!(obj_file, "v {} {} {}", vertex.0, vertex.1, vertex.2)?;
    // }

    // for index in &indices {
    //     writeln!(obj_file, "f {} {} {}", index[0] + 1, index[1] + 1, index[2] + 1)?;
    // }

    // obj_file.flush()?;
    // drop(obj_file);

    return Ok(());

    // let rec = rerun::RecordingStreamBuilder::new("glacier_dev").spawn()?;

    // rec.log(
    //     "terrain_heightmap",
    //     &rerun::Mesh3D::new(vertices).with_triangle_indices(indices),
    // )?;

    Ok(())
}
