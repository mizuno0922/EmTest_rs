use truck_modeling::*; 
use truck_polymesh::*; 
use truck_meshalgo::tessellation::MeshableShape;
use truck_meshalgo::tessellation::MeshedShape;
use cgmath::{Point3, Vector3};
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}

// `PolygonMesh`のヒープ上のポインタを返す
#[no_mangle]
pub extern "C" fn create_cube(size: f64) -> *mut c_void {
    let vertex = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let edge = builder::tsweep(&vertex, size * Vector3::unit_z());
    let face = builder::tsweep(&edge, size * Vector3::unit_x());
    let solid = builder::tsweep(&face, size * Vector3::unit_y());
    let mesh = solid.triangulation(0.01).to_polygon();

    // `PolygonMesh`をボックス化し、そのポインタを`*mut c_void`として返す
    Box::into_raw(Box::new(mesh)) as *mut c_void
}

// `create_cube`で作成された`PolygonMesh`のメモリを解放する関数
#[no_mangle]
pub extern "C" fn free_polygon_mesh(ptr: *mut c_void) {
    unsafe {
        // ポインタを`Box<PolygonMesh>`に変換してドロップすることでメモリを解放
        let _ = Box::from_raw(ptr as *mut PolygonMesh);
    }
}

// PolygonMeshから全ての頂点データを取得する関数
#[no_mangle]
pub extern "C" fn get_vertices(mesh: &PolygonMesh) -> *const f32 {
    // 頂点の位置データを取得
    let positions: &Vec<cgmath::Point3<f64>> = mesh.positions();

    // 頂点の位置データをf32の配列に変換
    let new_positions: Vec<f32> = positions.iter().flat_map(|p| vec![p.x as f32, p.y as f32, p.z as f32]).collect();

    // ベクタをボックス化してポインタを取得し、Rustの所有権を放棄
    let boxed_positions = new_positions.into_boxed_slice();
    let ptr = boxed_positions.as_ptr();
    std::mem::forget(boxed_positions);

    // C#側に渡すためのポインタを返す
    ptr
}

// PolygonMeshから全ての三角形の面インデックスを取得する関数
#[no_mangle]
pub extern "C" fn get_faces(mesh: &PolygonMesh) -> *const i32 {
    // Facesから三角形の面データを取得
    let faces: &Faces = mesh.faces();

    // 各三角形の頂点インデックスを格納するベクタを初期化
    let mut indices: Vec<i32> = Vec::new();

    // 三角形の面データからインデックスを抽出
    for face in faces.tri_faces().iter() {
        indices.push(face[0].pos as i32);
        indices.push(face[1].pos as i32);
        indices.push(face[2].pos as i32);
    }

    // ベクタをボックス化してポインタを取得し、Rustの所有権を放棄
    let boxed_indices = indices.into_boxed_slice();
    let ptr = boxed_indices.as_ptr();
    std::mem::forget(boxed_indices);

    // C#側に渡すためのポインタを返す
    ptr
}

#[no_mangle]
pub extern "C" fn get_vertex_count(mesh: &PolygonMesh) -> i32 {
    mesh.positions().len() as i32
}

#[no_mangle]
pub extern "C" fn get_face_count(mesh: &PolygonMesh) -> i32 {
    mesh.faces().len() as i32
}

#[no_mangle]
pub extern "C" fn free_vertices(ptr: *mut f32) {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn free_faces(ptr: *mut i32) {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}
