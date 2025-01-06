// import std.mem;

// struct Vec3 {
//     float x, y, z;
// };

// struct AxisAlignedBox {
//     Vec3 min;
//     Vec3 max;
// };

// u32 g_nodeSamplesPerSide;
// u32 g_densityMapNodeSamplesPerSide;

// u32 g_minMaxStackSize;
// u32 g_occluderGridStackSize;

// struct HeightfieldTreeNode {
//     AxisAlignedBox m_boundingBox;

//     bool nodeDisabled;
//     if (nodeDisabled)
//         return;

//     bool nodeDataSkipped;
//     bool nodeHasData;

//     if (!nodeDataSkipped && !nodeHasData)
//         return;

//     bool containsHoles;
//     bool nodePersistent;

//     if (nodeHasData && nodePersistent) {
//         u8 atlasData[(g_nodeSamplesPerSide * 2) * g_nodeSamplesPerSide];

//         if (g_minMaxStackSize > 0) {
//             u16 minMaxStackData[g_minMaxStackSize];
//         }

//         u16 occluderGridStackData[g_occluderGridStackSize];
//         u8 densityData[g_densityMapNodeSamplesPerSide * g_densityMapNodeSamplesPerSide];
//     } else if (nodeDataSkipped && g_minMaxStackSize > 0) {
//         u16 minMaxStackData[g_minMaxStackSize];
//     }

//     bool hasChildren;
//     if (!hasChildren)
//         return;

//     HeightfieldTreeNode children[4];
// };

// fn getMinMaxLevelOffset(u32 minMaxStackDepth) {
//     u32 result = 0;

//     u32 minMaxWidth = (1 << (minMaxStackDepth - 1));
//     for (u32 i = 0, i < minMaxStackDepth, i = i + 1) {
//         result = result + (minMaxWidth >> i) * (minMaxWidth >> i) * 2;
//     }

//     return result;
// };

// fn getOccluderLevelOffset(u32 occluderGridStackDepth) {
//     u32 result = 0;

//     u32 occluderGridWidthMinusOne = (1<<(occluderGridStackDepth-1));
//     for (u32 i = 0, i < occluderGridStackDepth, i = i + 1) {
//         u32 occluderGridLevelWidth = (occluderGridWidthMinusOne >> i) + 1;
//         result = result + occluderGridLevelWidth * occluderGridLevelWidth;
//     }

//     return result;
// };

// struct HeightfieldTree {
//     u32 nodeSamplesPerSide;
//     u32 resourceAtlasSampleCountX;
//     u32 resourceAtlasSampleCountY;
//     u32 blurriness;
//     float worldSizeY;
//     float physicsMetersPerSample;
//     s32 unk1;
//     bool unk2;
//     float unk3;
//     u32 minMaxStackDepth;
//     u32 occluderGridStackDepth;

//     g_nodeSamplesPerSide = nodeSamplesPerSide;

//     g_minMaxStackSize = getMinMaxLevelOffset(minMaxStackDepth);
//     g_occluderGridStackSize = getOccluderLevelOffset(occluderGridStackDepth);

//     u32 densityMapNodeSamplesPerSide;
//     u32 densityMapBorderWidth;
//     u32 densityMapNodeSamplesPerSidePot;
//     float densityMapResolutionRatio;

//     g_densityMapNodeSamplesPerSide = densityMapNodeSamplesPerSide;

//     u32 nodeCount;
//     u32 persistentNodeCount;
//     u32 persistentDedicatedServerNodeCount;
//     u32 skippedNodeCount;
//     u32 nodeBorderWidth;

//     HeightfieldTreeNode m_firstNode;
// };

// struct RasterTreeNode {
//     u8 rasterTreeType;
//     u32 rasterTreeLoadSize;

//     //u8 data[rasterTreeLoadSize];
//     HeightfieldTree tree;
// };

// struct TerrainStreamingTree {
//     u8 meta[16];

//     u32 unblurredSamplesPerNodeSidePot;
//     bool trackTextureDetailFalloff;
//     float invisibleDetailReductionFactor;
//     float occludedDetailReductionFactor;
//     u32 skipLevels;
//     u32 bakedLevels;
//     u32 nodeCount;
//     bool freeStreamingEnable;

//     RasterTreeNode firstNode;
//     //RasterTreeNode nodes[while(std::mem::read_unsigned($, 1) != 0xFF)];
// };

// TerrainStreamingTree tree @ 0x00;

use std::{
    fmt::Debug,
    num::Wrapping,
    ops::{Add, Mul},
};

use bytes::{Buf, BytesMut};
use glam::{Vec2, Vec3A};

const QUAD_TREE_NODE_CHILD_OFFSET_X: [u16; 4] = [0, 1, 1, 0];
const QUAD_TREE_NODE_CHILD_OFFSET_Y: [u16; 4] = [0, 0, 1, 1];

#[derive(Default, Clone, Debug)]
pub struct AxisAlignedBox {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl AxisAlignedBox {
    pub fn contains(&self, point: Vec3A) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn contains_2d(&self, point: Vec2) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.z <= point.y
            && point.y <= self.max.z
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct QuadtreeNodeId {
    index_x: u16,
    index_y: u16,
    pub level: u8,
}

#[derive(Default, Clone)]
pub struct HeightfieldTreeNode {
    id: QuadtreeNodeId,
    pub node_samples_per_side: u32,
    bounding_box: AxisAlignedBox,
    node_disabled: bool,
    contains_holes: bool,
    atlas_data: Vec<u16>,
    min_max_stack_data: Vec<u16>,
    occluder_grid_stack_data: Vec<u16>,
    density_data: Vec<u8>,
    has_children: bool,

    first_child_index: usize,
    pub children_have_data: bool,
    /// Only direct children have data
    world_scale_y: f32,
}

impl HeightfieldTreeNode {
    pub fn id(&self) -> &QuadtreeNodeId {
        &self.id
    }

    pub fn bounding_box(&self) -> &AxisAlignedBox {
        &self.bounding_box
    }

    pub fn has_data(&self) -> bool {
        !self.node_disabled && !self.atlas_data.is_empty()
    }

    pub fn sample_data(&self, x: u32, y: u32) -> u16 {
        let index = (y * self.node_samples_per_side + x) as usize;
        self.atlas_data[index.clamp(0, self.atlas_data.len() - 1)]
    }

    pub fn virtual_coords(&self, x: f32, z: f32) -> (u32, u32) {
        let x = (x - self.bounding_box.min.x) / (self.bounding_box.max.x - self.bounding_box.min.x);
        let z = (z - self.bounding_box.min.z) / (self.bounding_box.max.z - self.bounding_box.min.z);

        let x = (x * (self.node_samples_per_side - 1) as f32).round() as u32;
        let z = (z * (self.node_samples_per_side - 1) as f32).round() as u32;

        (x, z)
    }

    pub fn world_coords(&self, virtual_x: f32, virtual_z: f32) -> (f32, f32) {
        let x = self.bounding_box.min.x
            + (virtual_x / (self.node_samples_per_side - 1) as f32)
                * (self.bounding_box.max.x - self.bounding_box.min.x);
        let z = self.bounding_box.min.z
            + (virtual_z / (self.node_samples_per_side - 1) as f32)
                * (self.bounding_box.max.z - self.bounding_box.min.z);

        (x, z)
    }

    pub fn sample_virtual_to_world(&self, x: u32, z: u32) -> Vec3A {
        let height = self.sample_data(x, z) as f32 * self.world_scale_y;

        let world_coords = self.world_coords(x as f32, z as f32);
        Vec3A::new(world_coords.0, height, world_coords.1)
    }

    pub fn samples_to_world_coords(&self) -> Vec<Vec3A> {
        let mut points = Vec::with_capacity(self.atlas_data.len());

        let min = self.bounding_box.min;
        let max = self.bounding_box.max;

        let width = max.x - min.x;
        let depth = max.z - min.z;

        for (i, &height_sample) in self.atlas_data.iter().enumerate() {
            // 2D sample coordinates within the 137x137 grid
            let x = (i % self.node_samples_per_side as usize) as f32
                / (self.node_samples_per_side - 1) as f32;
            let z = (i / self.node_samples_per_side as usize) as f32
                / (self.node_samples_per_side - 1) as f32;

            // Convert to world space
            let world_x = min.x + x * width;
            let world_y = height_sample as f32 * self.world_scale_y;
            let world_z = min.z + z * depth;

            points.push(Vec3A::new(world_x, world_y, world_z));
        }

        points
    }

    pub fn find_closest_samples(&self, x: f32, z: f32) -> (Vec3A, Vec3A, Vec3A, Vec3A) {
        let (x, z) = self.virtual_coords(x, z);

        let top_left = self.sample_virtual_to_world(x, z);
        let top_right = self.sample_virtual_to_world(x + 1, z);
        let bottom_left = self.sample_virtual_to_world(x, z + 1);
        let bottom_right = self.sample_virtual_to_world(x + 1, z + 1);

        (top_left, top_right, bottom_left, bottom_right)
    }
}

impl Debug for HeightfieldTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeightfieldTreeNode")
            .field("id", &self.id)
            .field("bounding_box", &self.bounding_box)
            .field("atlas_count", &self.atlas_data.len())
            .finish()
    }
}

fn get_min_max_level_offset(minMaxStackDepth: u32) -> u32 {
    let mut result = 0;

    let minMaxWidth = (1 << (minMaxStackDepth - 1));
    for i in 0..minMaxStackDepth {
        result = result + (minMaxWidth >> i) * (minMaxWidth >> i) * 2;
    }

    result
}

fn get_occluder_level_offset(occluderGridStackDepth: u32) -> u32 {
    let mut result = 0;

    let occluder_grid_width_minus_one = (1 << (occluderGridStackDepth - 1));
    for i in 0..occluderGridStackDepth {
        let occluderGridLevelWidth = (occluder_grid_width_minus_one >> i) + 1;
        result = result + occluderGridLevelWidth * occluderGridLevelWidth;
    }

    result
}

#[derive(Debug)]
pub struct HeightfieldTree {
    node_samples_per_side: u32,
    resource_atlas_sample_count_x: u32,
    resource_atlas_sample_count_y: u32,
    blurriness: u32,
    world_size_y: f32,
    physics_meters_per_sample: f32,
    unk1: i32,
    unk2: bool,
    unk3: f32,
    min_max_stack_depth: u32,
    min_max_stack_size: u32,
    occluder_grid_stack_depth: u32,
    occluder_grid_stack_size: u32,
    density_map_node_samples_per_side: u32,
    density_map_border_width: u32,
    density_map_node_samples_per_side_pot: u32,
    density_map_resolution_ratio: f32,
    node_count: u32,
    persistent_node_count: u32,
    persistent_dedicated_server_node_count: u32,
    skipped_node_count: u32,
    node_border_width: u32,
    pub max_level: u8,
    pub nodes: Vec<HeightfieldTreeNode>,
}

impl HeightfieldTree {
    pub fn load(buf: &mut BytesMut) -> Self {
        let mut tree = HeightfieldTree {
            node_samples_per_side: buf.get_u32_le(),
            resource_atlas_sample_count_x: buf.get_u32_le(),
            resource_atlas_sample_count_y: buf.get_u32_le(),
            blurriness: buf.get_u32_le(),
            world_size_y: buf.get_f32_le(),
            physics_meters_per_sample: buf.get_f32_le(),
            unk1: buf.get_i32_le(),
            unk2: buf.get_u8() != 0,
            unk3: buf.get_f32_le(),
            min_max_stack_depth: buf.get_u32_le(),
            min_max_stack_size: 0,
            occluder_grid_stack_depth: buf.get_u32_le(),
            occluder_grid_stack_size: 0,
            density_map_node_samples_per_side: buf.get_u32_le(),
            density_map_border_width: buf.get_u32_le(),
            density_map_node_samples_per_side_pot: buf.get_u32_le(),
            density_map_resolution_ratio: buf.get_f32_le(),
            node_count: buf.get_u32_le(),
            persistent_node_count: buf.get_u32_le(),
            persistent_dedicated_server_node_count: buf.get_u32_le(),
            skipped_node_count: buf.get_u32_le(),
            node_border_width: buf.get_u32_le(),
            max_level: 0,
            nodes: Vec::new(),
        };

        println!("Node samples per side: {}", tree.node_samples_per_side);

        tree.min_max_stack_size = get_min_max_level_offset(tree.min_max_stack_depth);
        tree.occluder_grid_stack_size = get_occluder_level_offset(tree.occluder_grid_stack_depth);

        let root_node_id = QuadtreeNodeId {
            index_x: 0,
            index_y: 0,
            level: 0,
        };

        tree.nodes = vec![HeightfieldTreeNode::default(); tree.node_count as usize];
        tree.load_nodes(0, 1, buf, root_node_id);

        tree
    }

    /// Returns whether the node contains data
    fn load_nodes(
        &mut self,
        node_index: usize,
        mut first_free_node_index: usize,
        buf: &mut BytesMut,
        node_id: QuadtreeNodeId,
    ) -> bool {
        if node_id.level > self.max_level {
            self.max_level = node_id.level;
        }

        let node = &mut self.nodes[node_index];
        {
            node.id = node_id.clone();
            node.node_samples_per_side = self.node_samples_per_side;
            node.bounding_box = AxisAlignedBox {
                min: Vec3A::new(buf.get_f32_le(), buf.get_f32_le(), buf.get_f32_le()),
                max: Vec3A::new(buf.get_f32_le(), buf.get_f32_le(), buf.get_f32_le()),
            };
            node.node_disabled = false;
            node.contains_holes = false;
            node.atlas_data = Vec::new();
            node.min_max_stack_data = Vec::new();
            node.occluder_grid_stack_data = Vec::new();
            node.density_data = Vec::new();
            node.has_children = false;
            node.first_child_index = 0;
            node.children_have_data = false;
            node.world_scale_y = self.world_size_y / std::u16::MAX as f32;
        }

        node.node_disabled = buf.get_u8() != 0;
        if node.node_disabled {
            return false;
        }

        let node_data_skipped = buf.get_u8() != 0;
        let node_has_data = buf.get_u8() != 0;

        if node_has_data {
            // set CanBeStreamedIn flag
        } else if node_data_skipped {
        } else {
            return false;
        }

        node.contains_holes = buf.get_u8() != 0;

        //println!("Bounding box: {:?}", node.bounding_box);

        let node_persistent = buf.get_u8() != 0;
        if node_has_data && node_persistent {
            //println!("Loading node data for {:?}", node.id);

            node.atlas_data = (0..(((2 * self.node_samples_per_side as usize)
                * self.node_samples_per_side as usize)
                / 2))
                .map(|_| buf.get_u16_le())
                .collect();

            if self.min_max_stack_size > 0 {
                // read min_max_stack_data u16s
                node.min_max_stack_data = (0..self.min_max_stack_size)
                    .map(|_| buf.get_u16_le())
                    .collect();
            }

            // read occluder_grid_stack_data u16s
            node.occluder_grid_stack_data = (0..self.occluder_grid_stack_size)
                .map(|_| buf.get_u16_le())
                .collect();

            node.density_data = buf
                .copy_to_bytes(
                    self.density_map_node_samples_per_side as usize
                        * self.density_map_node_samples_per_side as usize,
                )
                .to_vec();
        } else if node_data_skipped && self.min_max_stack_size > 0 {
            //println!("Loading min_max_stack_data for {:?}", node.id);
            node.min_max_stack_data = (0..self.min_max_stack_size)
                .map(|_| buf.get_u16_le())
                .collect();
        }

        node.has_children = buf.get_u8() != 0;
        if node.has_children {
            node.first_child_index = first_free_node_index;
            let first_child_index = node.first_child_index;

            first_free_node_index += 4;

            let child_ids: Vec<(usize, QuadtreeNodeId)> = (0..4)
                .map(|i| {
                    (
                        i,
                        QuadtreeNodeId {
                            index_x: Wrapping(node.id.index_x)
                                .mul(Wrapping(2))
                                .add(Wrapping(QUAD_TREE_NODE_CHILD_OFFSET_X[i]))
                                .0,
                            index_y: Wrapping(node.id.index_y)
                                .mul(Wrapping(2))
                                .add(Wrapping(QUAD_TREE_NODE_CHILD_OFFSET_Y[i]))
                                .0,
                            level: node.id.level + 1,
                        },
                    )
                })
                .collect();

            let mut children_have_data = true;
            for (i, child_id) in child_ids {
                children_have_data &=
                    self.load_nodes(first_child_index + i, first_free_node_index, buf, child_id);
            }

            self.nodes[node_index].children_have_data = children_have_data;
        }

        node_has_data && node_persistent
    }

    pub fn root_node(&self) -> &HeightfieldTreeNode {
        &self.nodes[0]
    }

    pub fn get_best_node(&self, x: f32, z: f32) -> &HeightfieldTreeNode {
        let mut node = self.root_node();
        let mut best_node = None;

        while node.has_children {
            let mut found = false;

            for i in 0..4 {
                let child = &self.nodes[node.first_child_index as usize + i];
                if child.bounding_box.contains_2d(Vec2::new(x, z)) {
                    node = child;
                    found = true;
                    best_node = Some(node);
                    break;
                }
            }

            if !found {
                break;
            }
        }

        if best_node.is_none() {
            panic!("No node found for x: {}, z: {}", x, z);
        }

        best_node.unwrap()
    }
}

#[derive(Debug)]
pub enum RasterTreeNode {
    HeightfieldTree(HeightfieldTree),
    HeightfieldTree2(HeightfieldTree),
}

#[derive(Debug)]
pub struct TerrainStreamingTree {
    unblurred_samples_per_node_side_pot: u32,
    track_texture_detail_falloff: bool,
    invisible_detail_reduction_factor: f32,
    occluded_detail_reduction_factor: f32,
    skip_levels: u32,
    baked_levels: u32,
    node_count: u32,
    free_streaming_enable: bool,
    nodes: Vec<RasterTreeNode>,
    //first_node: RasterTreeNode,
    //RasterTreeNode nodes[while(std::mem::read_unsigned($, 1) != 0xFF)];
}

impl TerrainStreamingTree {
    pub fn load(buf: &mut BytesMut) -> Self {
        println!("buf len: {}", buf.len());

        let mut tree = TerrainStreamingTree {
            unblurred_samples_per_node_side_pot: buf.get_u32_le(),
            track_texture_detail_falloff: buf.get_u8() != 0,
            invisible_detail_reduction_factor: buf.get_f32_le(),
            occluded_detail_reduction_factor: buf.get_f32_le(),
            skip_levels: buf.get_u32_le(),
            baked_levels: buf.get_u32_le(),
            node_count: buf.get_u32_le(),
            free_streaming_enable: buf.get_u8() != 0,
            nodes: Vec::new(),
        };

        loop {
            //break;
            println!("buf len: {}", buf.len());

            let raster_tree_type = buf.get_u8();
            if raster_tree_type == 0xFF {
                break;
            }

            let raster_tree_load_size = buf.get_u32_le();
            let mut tree_buf = buf.split_to(raster_tree_load_size as usize);

            let raster_tree = match raster_tree_type {
                0 => Some(RasterTreeNode::HeightfieldTree(HeightfieldTree::load(
                    &mut tree_buf,
                ))),
                _ => {
                    println!("Unknown raster tree type: {}", raster_tree_type);
                    None
                }
            };

            if let Some(raster_tree) = raster_tree {
                tree.nodes.push(raster_tree);
            }
        }

        //tree.first_node = RasterTreeNode::load(buf);

        //while(std::mem::read_unsigned($, 1) != 0xFF) {
        //    tree.nodes.push(RasterTreeNode::load(buf));
        //}

        println!("Data remaining: {}", buf.remaining());

        tree
    }

    pub fn nodes(&self) -> &Vec<RasterTreeNode> {
        &self.nodes
    }
}

#[test]
fn test_terrain_streaming_tree() {
    // let data = include_bytes!("../../tests/data/hoth_01_terrain.streamingtree_win32.res");
    // let mut buf = BytesMut::from(&data[..]);
    // buf.advance(16);

    // let tree = TerrainStreamingTree::load(&mut buf);
    // println!("{:?}", tree);
}
