// Modules for optics ring generation
mod geometry;
mod stl_output;

use geometry::{RingType, RingParameters};
use stl_output::generate_stl_file;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeshData {
    vertices: Vec<f32>,  // Flattened vertex data [x,y,z,x,y,z,...]
    triangles: Vec<u32>, // Flattened triangle indices [i1,i2,i3,i1,i2,i3,...]
    vertex_count: usize,
    triangle_count: usize,
}

// Data structures for frontend communication
#[derive(Serialize, Deserialize, Debug)]
pub struct RingRequest {
    ring_type: String,
    outer_diameter: f32,
    inner_diameter: f32,
    output_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RingResponse {
    success: bool,
    message: String,
    filename: Option<String>,
    file_path: Option<String>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_ring(request: RingRequest) -> RingResponse {
    // Parse ring type
    let ring_type = match request.ring_type.parse::<RingType>() {
        Ok(rt) => rt,
        Err(e) => return RingResponse {
            success: false,
            message: format!("Invalid ring type: {}", e),
            filename: None,
            file_path: None,
        }
    };

    // Create ring parameters
    let params = match RingParameters::new(ring_type, request.outer_diameter, request.inner_diameter) {
        Ok(p) => p,
        Err(e) => return RingResponse {
            success: false,
            message: format!("Invalid parameters: {}", e),
            filename: None,
            file_path: None,
        }
    };

    // Determine output directory
    let output_dir = request.output_path.as_deref();

    // Generate STL file
    match generate_stl_file(&params, output_dir) {
        Ok(file_path) => RingResponse {
            success: true,
            message: format!("Successfully generated {} ring", ring_type),
            filename: Some(params.filename()),
            file_path: Some(file_path),
        },
        Err(e) => RingResponse {
            success: false,
            message: format!("Failed to generate STL: {}", e),
            filename: None,
            file_path: None,
        }
    }
}

#[tauri::command]
fn generate_mesh_preview(request: RingRequest) -> Result<MeshData, String> {
    println!("Starting mesh preview generation with request: {:?}", request);
    
    // Parse ring type
    let ring_type = match request.ring_type.parse::<RingType>() {
        Ok(rt) => rt,
        Err(e) => return Err(format!("Invalid ring type: {}", e))
    };
    println!("Parsed ring type: {:?}", ring_type);

    // Create ring parameters
    let params = match RingParameters::new(ring_type, request.outer_diameter, request.inner_diameter) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid parameters: {}", e))
    };
    println!("Created ring parameters successfully");

    // Generate mesh
    let mesh = match geometry::generate_ring_mesh(&params) {
        Ok(m) => m,
        Err(e) => return Err(format!("Failed to generate mesh: {}", e))
    };
    println!("Generated mesh with {} triangles", mesh.len());

    // Convert mesh to format suitable for Three.js
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();

    for (i, triangle) in mesh.iter().enumerate() {
        let base_index = (i * 3) as u32;
        
        // Add vertices as flat array [x,y,z,x,y,z,...]
        vertices.extend_from_slice(&[triangle.vertices[0][0], triangle.vertices[0][1], triangle.vertices[0][2]]);
        vertices.extend_from_slice(&[triangle.vertices[1][0], triangle.vertices[1][1], triangle.vertices[1][2]]);
        vertices.extend_from_slice(&[triangle.vertices[2][0], triangle.vertices[2][1], triangle.vertices[2][2]]);
        
        // Add triangle indices as flat array [i1,i2,i3,i1,i2,i3,...]
        triangles.extend_from_slice(&[base_index, base_index + 1, base_index + 2]);
    }

    let result = MeshData {
        vertex_count: vertices.len() / 3,  // Number of vertices (3 floats per vertex)
        triangle_count: triangles.len() / 3, // Number of triangles (3 indices per triangle)
        vertices,
        triangles,
    };
    
    println!("Created MeshData - vertex_count: {}, triangle_count: {}, vertices.len(): {}, triangles.len(): {}", 
             result.vertex_count, result.triangle_count, result.vertices.len(), result.triangles.len());
    
    if result.vertices.len() > 0 {
        println!("First few vertices: {:?}", &result.vertices[0..result.vertices.len().min(9)]);
    }
    if result.triangles.len() > 0 {
        println!("First few triangles: {:?}", &result.triangles[0..result.triangles.len().min(9)]);
    }
    
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, generate_ring, generate_mesh_preview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
