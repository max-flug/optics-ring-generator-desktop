use anyhow::Result;
use std::fs::File;
use std::path::Path;
use stl_io::write_stl;

use crate::geometry::{RingParameters, generate_ring_mesh};

/// Generate and save an STL file for the given ring parameters
pub fn generate_stl_file(params: &RingParameters, output_dir: Option<&str>) -> Result<String> {
    println!("Generating {} ring geometry...", params.ring_type);
    println!("  Outer diameter: {:.1}mm", params.outer_diameter);
    println!("  Inner diameter: {:.1}mm", params.inner_diameter);
    println!("  Height: {:.1}mm", params.height);
    
    // Generate the mesh
    let triangles = generate_ring_mesh(params)?;
    println!("  Generated {} triangles", triangles.len());
    
    // Determine output path
    let filename = params.filename();
    let output_path = if let Some(dir) = output_dir {
        Path::new(dir).join(&filename)
    } else {
        Path::new(&filename).to_path_buf()
    };
    
    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Write STL file
    let mut file = File::create(&output_path)?;
    write_stl(&mut file, triangles.iter())?;
    
    let output_path_str = output_path.to_string_lossy().to_string();
    println!("  STL file saved: {}", output_path_str);
    
    Ok(output_path_str)
}

/// Validate ring parameters for 3D printing
pub fn validate_for_printing(params: &RingParameters) -> Result<()> {
    let wall_thickness = (params.outer_diameter - params.inner_diameter) / 2.0;
    
    // Check minimum wall thickness for 3D printing (typically 1mm minimum)
    if wall_thickness < 1.0 {
        return Err(anyhow::anyhow!(
            "Wall thickness ({:.2}mm) is too thin for reliable 3D printing. Minimum recommended: 1.0mm",
            wall_thickness
        ));
    }
    
    // Check maximum reasonable size (300mm diameter should be enough for most printers)
    if params.outer_diameter > 300.0 {
        return Err(anyhow::anyhow!(
            "Outer diameter ({:.1}mm) exceeds typical 3D printer build volume",
            params.outer_diameter
        ));
    }
    
    // Check minimum practical size
    if params.inner_diameter < 5.0 {
        return Err(anyhow::anyhow!(
            "Inner diameter ({:.1}mm) is too small for practical use",
            params.inner_diameter
        ));
    }
    
    println!("✓ Ring parameters validated for 3D printing");
    println!("  Wall thickness: {:.2}mm", wall_thickness);
    
    Ok(())
}

/// Print estimated printing information
pub fn print_manufacturing_info(params: &RingParameters) {
    let wall_thickness = (params.outer_diameter - params.inner_diameter) / 2.0;
    let outer_radius = params.outer_diameter / 2.0;
    let inner_radius = params.inner_diameter / 2.0;
    
    // Estimate material volume (simplified calculation)
    let outer_volume = std::f32::consts::PI * outer_radius.powi(2) * params.height;
    let inner_volume = std::f32::consts::PI * inner_radius.powi(2) * params.height;
    let material_volume = outer_volume - inner_volume;
    
    println!("\n3D Printing Information:");
    println!("  Wall thickness: {:.2}mm", wall_thickness);
    println!("  Estimated material volume: {:.2}cm³", material_volume / 1000.0);
    
    // Printing recommendations
    println!("\nRecommended Print Settings:");
    match params.ring_type {
        crate::geometry::RingType::Convex => {
            println!("  - Layer height: 0.15-0.2mm for smooth curves");
            println!("  - Support: None required");
            println!("  - Orientation: Place flat on build plate");
        },
        crate::geometry::RingType::Concave => {
            println!("  - Layer height: 0.15-0.2mm for smooth curves");
            println!("  - Support: Light support for overhangs");
            println!("  - Orientation: Place flat on build plate");
        },
        crate::geometry::RingType::ThreePoint => {
            println!("  - Layer height: 0.2-0.3mm (structural print)");
            println!("  - Support: Minimal support for contact points");
            println!("  - Orientation: Place flat on build plate");
        },
    }
    
    // Material recommendations
    println!("  - Material: PLA or PETG for optical applications");
    println!("  - Infill: 100% for maximum stability");
}
