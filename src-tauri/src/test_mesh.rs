// Test file to debug mesh generation
use crate::{generate_mesh_preview, RingRequest};

pub fn test_mesh_generation() {
    let request = RingRequest {
        ring_type: "cylindrical".to_string(),
        outer_diameter: 20.0,
        inner_diameter: 10.0,
        output_path: None,
    };

    match generate_mesh_preview(request) {
        Ok(mesh_data) => {
            println!("Mesh generation successful!");
            println!("Vertex count: {}", mesh_data.vertex_count);
            println!("Triangle count: {}", mesh_data.triangle_count);
            println!("Vertices array length: {}", mesh_data.vertices.len());
            println!("Triangles array length: {}", mesh_data.triangles.len());
            
            if mesh_data.vertices.len() > 0 {
                println!("First 9 vertices: {:?}", &mesh_data.vertices[0..mesh_data.vertices.len().min(9)]);
            }
            if mesh_data.triangles.len() > 0 {
                println!("First 9 indices: {:?}", &mesh_data.triangles[0..mesh_data.triangles.len().min(9)]);
            }
        }
        Err(e) => {
            println!("Mesh generation failed: {}", e);
        }
    }
}
