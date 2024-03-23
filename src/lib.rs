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

// キューブを作成し、そのポインタをisizeとして返す
#[no_mangle]
pub extern "C" fn create_cube(size: f64) -> isize {
    let vertex = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let edge = builder::tsweep(&vertex, size * Vector3::unit_z());
    let face = builder::tsweep(&edge, size * Vector3::unit_x());
    let solid = builder::tsweep(&face, size * Vector3::unit_y());
    let mesh = solid.triangulation(0.01).to_polygon();

    Box::into_raw(Box::new(mesh)) as isize // ポインタをisizeに変換
}

// PolygonMeshのために割り当てられたメモリを解放
#[no_mangle]
pub unsafe extern "C" fn free_polygon_mesh(ptr: isize) {
    let _ = Box::from_raw(ptr as *mut PolygonMesh); // 自動的にドロップされる
}

// メッシュから頂点データを取得する関数
#[no_mangle]
pub unsafe extern "C" fn get_vertices(mesh_ptr: isize) -> *const f32 {
    let mesh = &*(mesh_ptr as *const PolygonMesh); // isizeをポインタに戻して参照外し
    let positions: &Vec<cgmath::Point3<f64>> = mesh.positions();
    let new_positions: Vec<f32> = positions.iter().flat_map(|p| vec![p.x as f32, p.y as f32, p.z as f32]).collect();
    let boxed_positions = new_positions.into_boxed_slice();
    let ptr = boxed_positions.as_ptr();
    std::mem::forget(boxed_positions); // Rustがボックス化されたスライスを自動的にクリーンアップしないようにする
    ptr
}

// メッシュから面データを取得する関数
#[no_mangle]
pub unsafe extern "C" fn get_faces(mesh_ptr: isize) -> *const i32 {
    let mesh = &*(mesh_ptr as *const PolygonMesh);
    let faces: &Faces = mesh.faces();
    let mut indices: Vec<i32> = Vec::new();
    for face in faces.tri_faces().iter() {
        indices.push(face[0].pos as i32);
        indices.push(face[1].pos as i32);
        indices.push(face[2].pos as i32);
    }
    let boxed_indices = indices.into_boxed_slice();
    let ptr = boxed_indices.as_ptr();
    std::mem::forget(boxed_indices);
    ptr
}

// 頂点の数を取得する関数
#[no_mangle]
pub unsafe extern "C" fn get_vertex_count(mesh_ptr: isize) -> i32 {
    let mesh = &*(mesh_ptr as *const PolygonMesh);
    mesh.positions().len() as i32
}

// 面の数を取得する関数
#[no_mangle]
pub unsafe extern "C" fn get_face_count(mesh_ptr: isize) -> i32 {
    let mesh = &*(mesh_ptr as *const PolygonMesh);
    mesh.faces().len() as i32
}

// 頂点データを解放する関数
#[no_mangle]
pub unsafe extern "C" fn free_vertices(ptr: isize) {
    let _ = Box::from_raw(ptr as *mut f32); // 自動的にドロップされる
}

// 面データを解放する関数
#[no_mangle]
pub unsafe extern "C" fn free_faces(ptr: isize) {
    let _ = Box::from_raw(ptr as *mut i32); // 自動的にドロップされる
}
