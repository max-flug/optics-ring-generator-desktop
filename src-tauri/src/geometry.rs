use anyhow::Result;
use nalgebra::{Point3, Vector3};
use stl_io::Triangle;

/// Represents the three types of support rings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RingType {
    Convex,   // CX
    Concave,  // CC
    ThreePoint, // 3P
}

impl std::fmt::Display for RingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RingType::Convex => write!(f, "CX"),
            RingType::Concave => write!(f, "CC"),
            RingType::ThreePoint => write!(f, "3P"),
        }
    }
}

impl std::str::FromStr for RingType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "CX" | "CONVEX" => Ok(RingType::Convex),
            "CC" | "CONCAVE" => Ok(RingType::Concave),
            "3P" | "THREEPOINT" | "THREE-POINT" => Ok(RingType::ThreePoint),
            _ => Err(anyhow::anyhow!("Invalid ring type: {}. Valid types are: CX, CC, 3P", s)),
        }
    }
}

/// Parameters for generating a support ring
#[derive(Debug, Clone)]
pub struct RingParameters {
    pub ring_type: RingType,
    pub outer_diameter: f32,
    pub inner_diameter: f32,
    pub height: f32,
}

impl RingParameters {
    pub fn new(ring_type: RingType, outer_diameter: f32, inner_diameter: f32) -> Result<Self> {
        if outer_diameter <= inner_diameter {
            return Err(anyhow::anyhow!("Outer diameter must be greater than inner diameter"));
        }
        if outer_diameter <= 0.0 || inner_diameter <= 0.0 {
            return Err(anyhow::anyhow!("Diameters must be positive"));
        }

        // Fixed height of 22mm as per technical drawing specification
        let height = 22.0;

        Ok(Self {
            ring_type,
            outer_diameter,
            inner_diameter,
            height,
        })
    }

    pub fn filename(&self) -> String {
        format!("{}-{:.1}.stl", self.ring_type, self.inner_diameter)
    }
}

/// Generate STL triangles for a support ring
pub fn generate_ring_mesh(params: &RingParameters) -> Result<Vec<Triangle>> {
    match params.ring_type {
        RingType::Convex => generate_convex_ring(params),
        RingType::Concave => generate_concave_ring(params),
        RingType::ThreePoint => generate_three_point_ring(params),
    }
}

fn generate_convex_ring(params: &RingParameters) -> Result<Vec<Triangle>> {
    let mut triangles = Vec::new();
    let segments = 64; // Number of segments for smooth curves
    
    let outer_radius = params.outer_diameter / 2.0;
    let inner_radius = params.inner_diameter / 2.0;
    
    // Static dimensions from technical drawing (in mm)
    let total_height = 22.0;
    let top_step_height = 10.0; // Height of top step
    let bottom_step_height = 10.0; // Height of bottom step  
    let middle_step_height = 2.0; // Small middle step
    
    // Edge dimensions
    let chamfer_depth = 1.5; // 30° chamfer depth
    let inner_step_width = 2.0; // Width of inner step
    
    // Generate the complex profile geometry
    for i in 0..segments {
        let angle1 = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) % segments) as f32 / segments as f32;
        
        // Generate the precise convex profile based on technical drawing
        add_convex_profile_segment(&mut triangles, angle1, angle2, inner_radius, outer_radius, 
                                 total_height, top_step_height, bottom_step_height, 
                                 middle_step_height, chamfer_depth, inner_step_width);
    }
    
    // Add horizontal surfaces (top and bottom faces)
    add_convex_horizontal_surfaces(&mut triangles, inner_radius, outer_radius, 
                                 total_height, top_step_height, bottom_step_height,
                                 middle_step_height, chamfer_depth, inner_step_width, segments);
    
    Ok(triangles)
}

fn generate_concave_ring(params: &RingParameters) -> Result<Vec<Triangle>> {
    let mut triangles = Vec::new();
    let segments = 64;
    
    let outer_radius = params.outer_diameter / 2.0;
    let inner_radius = params.inner_diameter / 2.0;
    let height = params.height;
    
    // Generate concave surface (curved outward)
    let curve_depth = (outer_radius - inner_radius) * 0.3;
    
    for i in 0..segments {
        let angle1 = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) % segments) as f32 / segments as f32;
        
        add_concave_segment(&mut triangles, angle1, angle2, inner_radius, outer_radius, height, curve_depth);
    }
    
    add_ring_base(&mut triangles, inner_radius, outer_radius, 0.0, segments);
    add_ring_top(&mut triangles, inner_radius, outer_radius, height, segments);
    
    Ok(triangles)
}

fn generate_three_point_ring(params: &RingParameters) -> Result<Vec<Triangle>> {
    let mut triangles = Vec::new();
    let segments = 64;
    
    let outer_radius = params.outer_diameter / 2.0;
    let inner_radius = params.inner_diameter / 2.0;
    let height = params.height;
    
    // Generate three contact points at 120-degree intervals
    let contact_angles = [0.0, 2.0 * std::f32::consts::PI / 3.0, 4.0 * std::f32::consts::PI / 3.0];
    let contact_width = std::f32::consts::PI / 12.0; // 15 degrees each
    
    for i in 0..segments {
        let angle1 = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) % segments) as f32 / segments as f32;
        
        // Check if this segment is near a contact point
        let is_contact_segment = contact_angles.iter().any(|&contact_angle| {
            let diff = (angle1 - contact_angle).abs().min((angle1 - contact_angle + 2.0 * std::f32::consts::PI).abs());
            diff < contact_width
        });
        
        if is_contact_segment {
            add_contact_segment(&mut triangles, angle1, angle2, inner_radius, outer_radius, height);
        } else {
            // Add minimal support structure between contact points
            add_minimal_segment(&mut triangles, angle1, angle2, inner_radius, outer_radius, height);
        }
    }
    
    add_ring_base(&mut triangles, inner_radius, outer_radius, 0.0, segments);
    add_ring_top(&mut triangles, inner_radius, outer_radius, height, segments);
    
    Ok(triangles)
}

// New precise convex ring geometry functions based on technical drawing

fn add_convex_profile_segment(triangles: &mut Vec<Triangle>, angle1: f32, angle2: f32,
                            inner_radius: f32, outer_radius: f32, total_height: f32,
                            top_step_height: f32, bottom_step_height: f32, 
                            middle_step_height: f32, chamfer_depth: f32, inner_step_width: f32) {
    
    // Define the precise profile points based on the technical drawing
    let profile_points = create_convex_profile_points(inner_radius, outer_radius, total_height,
                                                    top_step_height, bottom_step_height,
                                                    middle_step_height, chamfer_depth, inner_step_width);
    
    // Create vertical surfaces by connecting profile points between angles
    for i in 0..(profile_points.len() - 1) {
        let (r1, z1) = profile_points[i];
        let (r2, z2) = profile_points[i + 1];
        
        let p1 = Point3::new(r1 * angle1.cos(), r1 * angle1.sin(), z1);
        let p2 = Point3::new(r2 * angle1.cos(), r2 * angle1.sin(), z2);
        let p3 = Point3::new(r1 * angle2.cos(), r1 * angle2.sin(), z1);
        let p4 = Point3::new(r2 * angle2.cos(), r2 * angle2.sin(), z2);
        
        add_quad_triangles(triangles, p1, p2, p3, p4);
    }
}

fn create_convex_profile_points(inner_radius: f32, outer_radius: f32, total_height: f32,
                               _top_step_height: f32, _bottom_step_height: f32,
                               _middle_step_height: f32, _chamfer_depth: f32, _inner_step_width: f32) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    
    // Interpret the drawing correctly: the STRIPED areas are the actual ring material
    // This creates a ring with internal steps/recesses as shown in the hatched sections
    
    // The drawing shows the ring has a complex internal profile but simpler external profile
    // Height is 22mm, with internal steps creating the lens-holding geometry
    
    // External profile (outer surface) - relatively simple
    // Internal profile (inner surface) - complex with steps
    
    // Start from bottom outer edge
    points.push((outer_radius, 0.0));
    
    // Outer surface goes straight up (simple external profile)
    points.push((outer_radius, 22.0));
    
    // Top surface - from outer to inner
    // Based on the striped area, there's a recess at the top
    let top_recess_radius = inner_radius + 2.0;
    points.push((top_recess_radius, 22.0));
    
    // Drop down to create top recess (internal step)
    points.push((top_recess_radius, 12.0));
    
    // Internal step surface
    points.push((inner_radius, 12.0));
    
    // Inner surface down to bottom step
    points.push((inner_radius, 10.0));
    
    // Bottom step (internal)
    points.push((top_recess_radius, 10.0));
    
    // Down to bottom
    points.push((top_recess_radius, 0.0));
    
    // Bottom surface back to outer edge
    points.push((outer_radius, 0.0));
    
    points
}

fn add_convex_horizontal_surfaces(triangles: &mut Vec<Triangle>, inner_radius: f32, outer_radius: f32,
                                _total_height: f32, _top_step_height: f32, _bottom_step_height: f32,
                                _middle_step_height: f32, _chamfer_depth: f32, _inner_step_width: f32, segments: usize) {
    
    let top_recess_radius = inner_radius + 2.0;
    
    // Bottom face - full ring (outer to inner recess)
    add_annular_surface(triangles, top_recess_radius, outer_radius, 0.0, segments, false);
    
    // Bottom step surface at 10mm height
    add_annular_surface(triangles, inner_radius, top_recess_radius, 10.0, segments, true);
    
    // Top step surface at 12mm height  
    add_annular_surface(triangles, inner_radius, top_recess_radius, 12.0, segments, false);
    
    // Top face at 22mm height - with recess
    add_annular_surface(triangles, top_recess_radius, outer_radius, 22.0, segments, true);
}

fn add_annular_surface(triangles: &mut Vec<Triangle>, inner_r: f32, outer_r: f32, z: f32, segments: usize, face_up: bool) {
    for i in 0..segments {
        let angle1 = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) % segments) as f32 / segments as f32;
        
        let p1 = Point3::new(inner_r * angle1.cos(), inner_r * angle1.sin(), z);
        let p2 = Point3::new(outer_r * angle1.cos(), outer_r * angle1.sin(), z);
        let p3 = Point3::new(inner_r * angle2.cos(), inner_r * angle2.sin(), z);
        let p4 = Point3::new(outer_r * angle2.cos(), outer_r * angle2.sin(), z);
        
        if face_up {
            add_triangle(triangles, p1, p2, p3);
            add_triangle(triangles, p2, p4, p3);
        } else {
            add_triangle(triangles, p1, p3, p2);
            add_triangle(triangles, p2, p3, p4);
        }
    }
}

fn add_convex_segment(triangles: &mut Vec<Triangle>, angle1: f32, angle2: f32, 
                     inner_radius: f32, outer_radius: f32, _height: f32, curve_depth: f32) {
    // Create curved surface that curves inward (convex from lens perspective)
    let steps = 8;
    for step in 0..steps {
        let t1 = step as f32 / steps as f32;
        let t2 = (step + 1) as f32 / steps as f32;
        
        let r1 = inner_radius + (outer_radius - inner_radius) * t1;
        let r2 = inner_radius + (outer_radius - inner_radius) * t2;
        
        // Curve calculation - convex shape
        let z_offset1 = curve_depth * (1.0 - (2.0 * t1 - 1.0).powi(2));
        let z_offset2 = curve_depth * (1.0 - (2.0 * t2 - 1.0).powi(2));
        
        let p1 = Point3::new(r1 * angle1.cos(), r1 * angle1.sin(), z_offset1);
        let p2 = Point3::new(r2 * angle1.cos(), r2 * angle1.sin(), z_offset2);
        let p3 = Point3::new(r1 * angle2.cos(), r1 * angle2.sin(), z_offset1);
        let p4 = Point3::new(r2 * angle2.cos(), r2 * angle2.sin(), z_offset2);
        
        add_quad_triangles(triangles, p1, p2, p3, p4);
    }
}

fn add_concave_segment(triangles: &mut Vec<Triangle>, angle1: f32, angle2: f32,
                      inner_radius: f32, outer_radius: f32, _height: f32, curve_depth: f32) {
    // Create curved surface that curves outward (concave from lens perspective)
    let steps = 8;
    for step in 0..steps {
        let t1 = step as f32 / steps as f32;
        let t2 = (step + 1) as f32 / steps as f32;
        
        let r1 = inner_radius + (outer_radius - inner_radius) * t1;
        let r2 = inner_radius + (outer_radius - inner_radius) * t2;
        
        // Curve calculation - concave shape (inverted)
        let z_offset1 = -curve_depth * (1.0 - (2.0 * t1 - 1.0).powi(2));
        let z_offset2 = -curve_depth * (1.0 - (2.0 * t2 - 1.0).powi(2));
        
        let p1 = Point3::new(r1 * angle1.cos(), r1 * angle1.sin(), z_offset1);
        let p2 = Point3::new(r2 * angle1.cos(), r2 * angle1.sin(), z_offset2);
        let p3 = Point3::new(r1 * angle2.cos(), r1 * angle2.sin(), z_offset1);
        let p4 = Point3::new(r2 * angle2.cos(), r2 * angle2.sin(), z_offset2);
        
        add_quad_triangles(triangles, p1, p2, p3, p4);
    }
}

fn add_contact_segment(triangles: &mut Vec<Triangle>, angle1: f32, angle2: f32,
                      inner_radius: f32, outer_radius: f32, _height: f32) {
    // Full contact surface for three-point support
    let p1 = Point3::new(inner_radius * angle1.cos(), inner_radius * angle1.sin(), 0.0);
    let p2 = Point3::new(outer_radius * angle1.cos(), outer_radius * angle1.sin(), 0.0);
    let p3 = Point3::new(inner_radius * angle2.cos(), inner_radius * angle2.sin(), 0.0);
    let p4 = Point3::new(outer_radius * angle2.cos(), outer_radius * angle2.sin(), 0.0);
    
    add_quad_triangles(triangles, p1, p2, p3, p4);
}

fn add_minimal_segment(triangles: &mut Vec<Triangle>, angle1: f32, angle2: f32,
                      inner_radius: f32, outer_radius: f32, height: f32) {
    // Minimal structure between contact points - just outer ring connection
    let reduced_radius = inner_radius + (outer_radius - inner_radius) * 0.8;
    
    let p1 = Point3::new(reduced_radius * angle1.cos(), reduced_radius * angle1.sin(), height * 0.5);
    let p2 = Point3::new(outer_radius * angle1.cos(), outer_radius * angle1.sin(), 0.0);
    let p3 = Point3::new(reduced_radius * angle2.cos(), reduced_radius * angle2.sin(), height * 0.5);
    let p4 = Point3::new(outer_radius * angle2.cos(), outer_radius * angle2.sin(), 0.0);
    
    add_quad_triangles(triangles, p1, p2, p3, p4);
}

fn add_ring_base(triangles: &mut Vec<Triangle>, inner_radius: f32, outer_radius: f32, z: f32, segments: usize) {
    for i in 0..segments {
        let angle1 = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) % segments) as f32 / segments as f32;
        
        let p1 = Point3::new(inner_radius * angle1.cos(), inner_radius * angle1.sin(), z);
        let p2 = Point3::new(outer_radius * angle1.cos(), outer_radius * angle1.sin(), z);
        let p3 = Point3::new(inner_radius * angle2.cos(), inner_radius * angle2.sin(), z);
        let p4 = Point3::new(outer_radius * angle2.cos(), outer_radius * angle2.sin(), z);
        
        // Add triangles for ring base (facing down)
        add_triangle(triangles, p1, p3, p2);
        add_triangle(triangles, p2, p3, p4);
    }
}

fn add_ring_top(triangles: &mut Vec<Triangle>, inner_radius: f32, outer_radius: f32, z: f32, segments: usize) {
    for i in 0..segments {
        let angle1 = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let angle2 = 2.0 * std::f32::consts::PI * ((i + 1) % segments) as f32 / segments as f32;
        
        let p1 = Point3::new(inner_radius * angle1.cos(), inner_radius * angle1.sin(), z);
        let p2 = Point3::new(outer_radius * angle1.cos(), outer_radius * angle1.sin(), z);
        let p3 = Point3::new(inner_radius * angle2.cos(), inner_radius * angle2.sin(), z);
        let p4 = Point3::new(outer_radius * angle2.cos(), outer_radius * angle2.sin(), z);
        
        // Add triangles for ring top (facing up)
        add_triangle(triangles, p1, p2, p3);
        add_triangle(triangles, p2, p4, p3);
    }
}

fn add_quad_triangles(triangles: &mut Vec<Triangle>, p1: Point3<f32>, p2: Point3<f32>, 
                     p3: Point3<f32>, p4: Point3<f32>) {
    // Split quad into two triangles
    add_triangle(triangles, p1, p2, p3);
    add_triangle(triangles, p2, p4, p3);
}

fn add_triangle(triangles: &mut Vec<Triangle>, p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) {
    // Calculate normal
    let edge1 = Vector3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
    let edge2 = Vector3::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
    let normal = edge1.cross(&edge2).normalize();
    
    triangles.push(Triangle {
        normal: stl_io::Vector::new([normal.x, normal.y, normal.z]),
        vertices: [
            stl_io::Vector::new([p1.x, p1.y, p1.z]),
            stl_io::Vector::new([p2.x, p2.y, p2.z]),
            stl_io::Vector::new([p3.x, p3.y, p3.z]),
        ],
    });
}
