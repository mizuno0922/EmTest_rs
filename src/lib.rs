use truck_modeling::*; 
use truck_polymesh::*; 
use truck_meshalgo::tessellation::MeshableShape;
use truck_meshalgo::tessellation::MeshedShape;
use cgmath::{Point3 as CGPoint3, Vector3};
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}

#[repr(C)] // C言語のメモリレイアウト互換性を保証
pub struct Point3 {
    pub x: f64, // x座標
    pub y: f64, // y座標
    pub z: f64, // z座標
}

impl Point3 {
    // Rust側で使うための新しい`Point3`インスタンスを作成するコンストラクタ
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }
}

// `extern "C"`でC言語のインターフェースを提供
// この関数は、Point3のインスタンスを作成し、そのポインタをisizeとして返します。
#[no_mangle] // シンボル名のマングリングを防止
pub extern "C" fn construct_point3(x: f64, y: f64, z: f64) -> isize {
    let p3 = Point3::new(x, y, z);
    // ヒープ上に`Point3`インスタンスを生成し、そのポインタをisizeにキャストして返す
    Box::into_raw(Box::new(p3)) as isize
}

// この関数は、isizeとして渡されたポインタを`Point3`のポインタに戻し、
// 対象のメモリを解放します。
#[no_mangle]
pub extern "C" fn point3_free(pptr: isize) {
    if pptr != 0 {
        unsafe { // unsafeブロックを追加
            let _ = Box::from_raw(pptr as *mut Point3); // 自動的にドロップされる
        }
    }
}

// x座標の値を取得する関数
#[no_mangle]
pub extern "C" fn point3_get_x(pptr: isize) -> f64 {
    if pptr != 0 {
        unsafe { // unsafeブロックを追加
            let pt = &*(pptr as *const Point3);
            pt.x as f64
        }
    } else {
        0.0 // ポインタがnull（0）の場合、デフォルトとして0.0を返す
    }
}

// y座標の値を取得する関数
#[no_mangle]
pub extern "C" fn point3_get_y(pptr: isize) -> f64 {
    if pptr != 0 {
        unsafe { // unsafeブロックを追加
            let pt = &*(pptr as *const Point3);
            pt.y as f64
        }
    } else {
        0.0
    }
}

// z座標の値を取得する関数
#[no_mangle]
pub extern "C" fn point3_get_z(pptr: isize) -> f64 {
    if pptr != 0 {
        unsafe { // unsafeブロックを追加
            let pt = &*(pptr as *const Point3);
            pt.z as f64
        }
    } else {
        0.0
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_ptx(pptr: isize) {
    let _ = Box::from_raw(pptr as *mut f64); // 自動的にドロップされる
}

#[no_mangle]
pub unsafe extern "C" fn free_pty(pptr: isize) {
    let _ = Box::from_raw(pptr as *mut f64); // 自動的にドロップされる
}

#[no_mangle]
pub unsafe extern "C" fn free_ptz(pptr: isize) {
    let _ = Box::from_raw(pptr as *mut f64); // 自動的にドロップされる
}

// キューブを作成し、そのポインタをisizeとして返す
#[no_mangle]
pub extern "C" fn create_cube(size: f64) -> isize {
    let vertex = builder::vertex(CGPoint3::new(0.0, 0.0, 0.0));
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
