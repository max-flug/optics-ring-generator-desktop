// Three.js imports
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { STLLoader } from 'three/addons/loaders/STLLoader.js';

// Wait for Tauri to be ready and import APIs
let invoke, open;

async function initializeTauri() {
  try {
    // Wait for Tauri to be available
    if (typeof window.__TAURI__ === 'undefined') {
      console.error('Tauri APIs not available');
      return false;
    }
    
    const { invoke: tauriInvoke } = window.__TAURI__.core;
    const { open: tauriOpen } = window.__TAURI__.dialog;
    
    invoke = tauriInvoke;
    open = tauriOpen;
    
    console.log('Tauri APIs initialized successfully');
    return true;
  } catch (error) {
    console.error('Failed to initialize Tauri APIs:', error);
    return false;
  }
}

// DOM elements
let ringForm;
let resultSection;
let successMessage;
let errorMessage;
let successText;
let errorText;
let filenameSpan;
let filepathSpan;
let outputPathInput;
let browseBtn;

// Current selected output path
let selectedOutputPath = null;

// 3D Preview variables
let scene, camera, renderer, controls;
let currentMesh = null;
let previewSection, previewCanvas, previewContainer;
let vertexCountSpan, triangleCountSpan, dimensionsSpan;

// Open file dialog to select save location
async function selectOutputLocation() {
  console.log('Browse button clicked - selectOutputLocation called');
  
  try {
    console.log('Attempting to open file dialog...');
    
    if (!open) {
      throw new Error('Dialog API not available');
    }
    
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select folder to save STL file"
    });

    console.log('Dialog result:', selected);

    if (selected) {
      selectedOutputPath = selected;
      outputPathInput.value = selected;
      console.log('Path selected:', selected);
    } else {
      console.log('No path selected (user cancelled)');
    }
  } catch (error) {
    console.error('Failed to open file dialog:', error);
    showError(`Failed to open file dialog: ${error.message}`);
  }
}

// Initialize 3D viewer
function init3DViewer() {
  // Scene
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0);

  // Camera
  camera = new THREE.PerspectiveCamera(75, previewCanvas.clientWidth / previewCanvas.clientHeight, 0.1, 1000);
  camera.position.set(50, 50, 50);

  // Renderer
  renderer = new THREE.WebGLRenderer({ canvas: previewCanvas, antialias: true });
  renderer.setSize(previewCanvas.clientWidth, previewCanvas.clientHeight);
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  // Lights
  const ambientLight = new THREE.AmbientLight(0x404040, 0.6);
  scene.add(ambientLight);

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
  directionalLight.position.set(50, 50, 25);
  directionalLight.castShadow = true;
  directionalLight.shadow.mapSize.width = 2048;
  directionalLight.shadow.mapSize.height = 2048;
  scene.add(directionalLight);

  // Controls
  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.dampingFactor = 0.05;

  console.log('3D viewer initialized');
}

// Generate and show 3D preview
async function generate3DPreview(ringType, outerDiameter, innerDiameter) {
  try {
    console.log('Generating 3D preview...');
    
    // Show preview section
    previewSection.classList.remove('hidden');
    
    // Get mesh data from Rust backend
    const meshData = await invoke("generate_mesh_preview", {
      request: {
        ring_type: ringType,
        outer_diameter: outerDiameter,
        inner_diameter: innerDiameter,
      }
    });

    console.log('Mesh data received:', meshData);

    // Update info panel with model dimensions
    const modelDimensionsEl = document.getElementById('model-dimensions');
    if (modelDimensionsEl) modelDimensionsEl.textContent = `${outerDiameter}mm × ${innerDiameter}mm`;

    // Initialize 3D viewer
    init3DViewer();

    // Create geometry from mesh data using a more robust approach
    console.log('Mesh data structure:', meshData);
    console.log('Vertices length:', meshData.vertices?.length);
    console.log('Triangles length:', meshData.triangles?.length);
    
    // Check if data is valid
    if (!meshData.vertices || !meshData.triangles) {
      throw new Error('Invalid mesh data: missing vertices or triangles');
    }
    
    if (meshData.vertices.length === 0 || meshData.triangles.length === 0) {
      throw new Error('Invalid mesh data: empty vertices or triangles arrays');
    }

    // Create geometry using Three.js BufferGeometry in a more direct way
    const geometry = new THREE.BufferGeometry();
    
    // Method 1: Try using setFromPoints for vertices first
    try {
      console.log('Method 1: Using setFromPoints approach');
      
      // Convert flat vertex array to Vector3 points
      const points = [];
      for (let i = 0; i < meshData.vertices.length; i += 3) {
        points.push(new THREE.Vector3(
          meshData.vertices[i],
          meshData.vertices[i + 1], 
          meshData.vertices[i + 2]
        ));
      }
      
      console.log('Created', points.length, 'Vector3 points');
      
      // Set the position attribute from points
      geometry.setFromPoints(points);
      
      // Set indices manually
      const indexArray = [];
      for (let i = 0; i < meshData.triangles.length; i++) {
        indexArray.push(meshData.triangles[i]);
      }
      
      geometry.setIndex(indexArray);
      console.log('Successfully created geometry using setFromPoints');
      
    } catch (error) {
      console.warn('Method 1 failed:', error);
      console.log('Method 2: Trying manual BufferAttribute creation');
      
      try {
        // Method 2: Create BufferAttribute manually with explicit array conversion
        const positionArray = [];
        for (let i = 0; i < meshData.vertices.length; i++) {
          positionArray[i] = Number(meshData.vertices[i]);
        }
        
        const indexArray = [];
        for (let i = 0; i < meshData.triangles.length; i++) {
          indexArray[i] = Number(meshData.triangles[i]);
        }
        
        console.log('Created position array with', positionArray.length, 'elements');
        console.log('Created index array with', indexArray.length, 'elements');
        
        // Create the typed arrays explicitly
        const vertices = new Float32Array(positionArray);
        const indices = new Uint32Array(indexArray);
        
        console.log('TypedArrays created - vertices:', vertices.constructor.name, 'indices:', indices.constructor.name);
        
        // Clear geometry and set attributes
        geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
        geometry.setIndex(indices);
        
        console.log('Successfully created geometry using manual BufferAttribute');
        
      } catch (error2) {
        console.error('Method 2 also failed:', error2);
        
        // Method 3: Fallback to basic triangle creation
        console.log('Method 3: Creating simple test geometry');
        
        // Create a simple test triangle to verify Three.js is working
        const testVertices = new Float32Array([
          -1.0, -1.0, 0.0,
           1.0, -1.0, 0.0,
           0.0,  1.0, 0.0
        ]);
        
        const testIndices = new Uint16Array([0, 1, 2]);
        
        geometry.setAttribute('position', new THREE.BufferAttribute(testVertices, 3));
        geometry.setIndex(testIndices);
        
        console.log('Created fallback test triangle');
      }
    }
    geometry.computeVertexNormals();

    // Create material
    const material = new THREE.MeshLambertMaterial({ 
      color: 0x4A90E2,
      side: THREE.DoubleSide 
    });

    // Remove previous mesh
    if (currentMesh) {
      scene.remove(currentMesh);
    }

    // Create and add new mesh
    currentMesh = new THREE.Mesh(geometry, material);
    currentMesh.castShadow = true;
    currentMesh.receiveShadow = true;
    scene.add(currentMesh);

    // Center the camera on the object
    const box = new THREE.Box3().setFromObject(currentMesh);
    const center = box.getCenter(new THREE.Vector3());
    const size = box.getSize(new THREE.Vector3());
    
    controls.target.copy(center);
    const maxDim = Math.max(size.x, size.y, size.z);
    camera.position.set(center.x + maxDim, center.y + maxDim, center.z + maxDim);
    controls.update();

    // Add control event listeners
    setupPreviewControls();

    // Start render loop
    animate();

    console.log('3D preview loaded successfully');
    
  } catch (error) {
    console.error('Failed to generate 3D preview:', error);
    previewContainer.innerHTML = `<div style="color: red; padding: 20px;">Failed to generate 3D preview: ${error.message}</div>`;
  }
}

// Setup preview control buttons
function setupPreviewControls() {
  document.getElementById('preview-rotate')?.addEventListener('click', () => {
    controls.autoRotate = !controls.autoRotate;
  });

  document.getElementById('preview-zoom-in')?.addEventListener('click', () => {
    camera.position.multiplyScalar(0.9);
    controls.update();
  });

  document.getElementById('preview-zoom-out')?.addEventListener('click', () => {
    camera.position.multiplyScalar(1.1);
    controls.update();
  });

  document.getElementById('preview-reset')?.addEventListener('click', () => {
    if (currentMesh) {
      const box = new THREE.Box3().setFromObject(currentMesh);
      const center = box.getCenter(new THREE.Vector3());
      const size = box.getSize(new THREE.Vector3());
      
      controls.target.copy(center);
      const maxDim = Math.max(size.x, size.y, size.z);
      camera.position.set(center.x + maxDim, center.y + maxDim, center.z + maxDim);
      controls.autoRotate = false;
      controls.update();
    }
  });
}

// Animation loop
function animate() {
  requestAnimationFrame(animate);
  
  if (controls) {
    controls.update();
  }
  
  if (renderer && scene && camera) {
    renderer.render(scene, camera);
  }
}

// Handle window resize
function onWindowResize() {
  if (camera && renderer && previewCanvas) {
    camera.aspect = previewCanvas.clientWidth / previewCanvas.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(previewCanvas.clientWidth, previewCanvas.clientHeight);
  }
}

// Generate ring STL file
async function generateRing(ringType, outerDiameter, innerDiameter, outputPath) {
  try {
    showLoading();
    
    const response = await invoke("generate_ring", {
      request: {
        ring_type: ringType,
        outer_diameter: outerDiameter,
        inner_diameter: innerDiameter,
        output_path: outputPath,
      }
    });

    if (response.success) {
      showSuccess(response);
    } else {
      showError(response.message);
    }
  } catch (error) {
    showError(`Failed to generate ring: ${error}`);
  }
}

function showLoading() {
  resultSection.classList.remove('hidden');
  successMessage.classList.add('hidden');
  errorMessage.classList.add('hidden');
  
  // You could add a loading spinner here if desired
}

function showSuccess(response) {
  successText.textContent = response.message;
  filenameSpan.textContent = response.filename || 'N/A';
  filepathSpan.textContent = response.file_path || 'N/A';
  
  successMessage.classList.remove('hidden');
  errorMessage.classList.add('hidden');
}

function showError(message) {
  errorText.textContent = message;
  errorMessage.classList.remove('hidden');
  successMessage.classList.add('hidden');
}

function validateForm() {
  const ringType = document.getElementById('ring-type').value;
  const outerDiameter = parseFloat(document.getElementById('outer-diameter').value);
  const innerDiameter = parseFloat(document.getElementById('inner-diameter').value);

  if (!ringType) {
    throw new Error('Please select a ring type');
  }

  if (isNaN(outerDiameter) || outerDiameter <= 0) {
    throw new Error('Please enter a valid outer diameter');
  }

  if (isNaN(innerDiameter) || innerDiameter <= 0) {
    throw new Error('Please enter a valid inner diameter');
  }

  if (outerDiameter <= innerDiameter) {
    throw new Error('Outer diameter must be greater than inner diameter');
  }

  const wallThickness = (outerDiameter - innerDiameter) / 2;
  if (wallThickness < 1.0) {
    throw new Error(`Wall thickness (${wallThickness.toFixed(2)}mm) is too thin. Minimum recommended: 1.0mm`);
  }

  return { ringType, outerDiameter, innerDiameter };
}

window.addEventListener("DOMContentLoaded", async () => {
  // Initialize Tauri APIs first
  const tauriReady = await initializeTauri();
  if (!tauriReady) {
    document.body.innerHTML = '<div style="padding: 20px; color: red;">Error: Tauri APIs not available. Please ensure you are running this in a Tauri application.</div>';
    return;
  }

  // Get DOM elements
  ringForm = document.getElementById('ring-form');
  resultSection = document.getElementById('result-section');
  successMessage = document.getElementById('success-message');
  errorMessage = document.getElementById('error-message');
  successText = document.getElementById('success-text');
  errorText = document.getElementById('error-text');
  filenameSpan = document.getElementById('filename');
  filepathSpan = document.getElementById('filepath');
  outputPathInput = document.getElementById('output-path');
  browseBtn = document.getElementById('browse-btn');
  
  // 3D Preview elements
  previewSection = document.getElementById('preview-section');
  previewCanvas = document.getElementById('preview-canvas');
  previewContainer = document.getElementById('preview-container');
  
  // New UI elements
  const loadingState = document.getElementById('loading-state');
  const modelDimensions = document.getElementById('model-dimensions');
  const actionPanel = document.getElementById('action-panel');
  const saveStlBtn = document.getElementById('save-stl-btn');

  console.log('DOM elements initialized');

  // Check if browse button exists
  if (!browseBtn) {
    console.error('Browse button not found!');
    return;
  }
  
  console.log('Browse button found:', browseBtn);

  // Add window resize listener for 3D viewer
  window.addEventListener('resize', onWindowResize);

  // Add browse button handler
  browseBtn.addEventListener('click', (e) => {
    console.log('Browse button click event fired');
    e.preventDefault();
    selectOutputLocation();
  });
  
  console.log('Browse button event listener attached');

  // Add form submit handler
  ringForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    try {
      const { ringType, outerDiameter, innerDiameter } = validateForm();
      
      // Show loading state
      loadingState.style.display = 'flex';
      actionPanel.style.display = 'none';
      
      // First generate and show 3D preview
      await generate3DPreview(ringType, outerDiameter, innerDiameter);
      
      // Hide loading state and show action panel
      loadingState.style.display = 'none';
      actionPanel.style.display = 'block';
      
    } catch (error) {
      loadingState.style.display = 'none';
      showError(error.message);
    }
  });

  // Add save STL button handler
  saveStlBtn.addEventListener('click', async () => {
    try {
      const { ringType, outerDiameter, innerDiameter } = validateForm();
      await generateRing(ringType, outerDiameter, innerDiameter, selectedOutputPath);
    } catch (error) {
      showError(error.message);
    }
  });

  // Add input validation helpers
  const outerDiameterInput = document.getElementById('outer-diameter');
  const innerDiameterInput = document.getElementById('inner-diameter');

  function updateMaxInnerDiameter() {
    const outerValue = parseFloat(outerDiameterInput.value);
    if (!isNaN(outerValue)) {
      innerDiameterInput.max = (outerValue - 2).toString(); // Leave at least 1mm wall thickness
    }
  }

  outerDiameterInput.addEventListener('input', updateMaxInnerDiameter);
  
  // Real-time validation display
  const inputs = [outerDiameterInput, innerDiameterInput];
  inputs.forEach(input => {
    input.addEventListener('input', () => {
      try {
        validateForm();
        input.classList.remove('error');
      } catch (error) {
        input.classList.add('error');
      }
    });
  });

  // Add keyboard shortcuts for window management
  document.addEventListener('keydown', (event) => {
    // F11 - Toggle fullscreen
    if (event.key === 'F11') {
      event.preventDefault();
      if (invoke) {
        invoke('toggle_fullscreen').catch(console.error);
      }
    }
    
    // Ctrl+M - Toggle maximize
    if (event.ctrlKey && event.key === 'm') {
      event.preventDefault();
      if (invoke) {
        invoke('set_window_maximized', { maximized: true }).catch(console.error);
      }
    }
    
    // Escape - Exit fullscreen
    if (event.key === 'Escape') {
      if (invoke) {
        invoke('toggle_fullscreen').then(isFullscreen => {
          if (isFullscreen) {
            // If still fullscreen after toggle, force exit
            invoke('toggle_fullscreen').catch(console.error);
          }
        }).catch(console.error);
      }
    }
  });
});
