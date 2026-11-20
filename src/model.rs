//! A stand-in model: a closed box, for a game whose own model can't be read (yet).
use engine_core::glam::Vec3;
use engine_core::model::{Lighting, MaterialDesc, MeshData};
use engine_core::{Model, ModelSource};

/// A box from corner `lo` to corner `hi` (in the model's space: its forward +z, its up +y), one joint, coloured
/// `color` (sRGB), its front face (+z) tinted `front` so that it reads which way it faces.
pub fn box_model(lo: Vec3, hi: Vec3, color: [f32; 4], front: [f32; 4]) -> Model {
    let mut mesh = MeshData::default();
    // (each face its own four corners, wound counter-clockwise from outside)
    let faces: [(Vec3, [Vec3; 4]); 6] = [
        (Vec3::Y, [Vec3::new(lo.x, hi.y, lo.z), Vec3::new(lo.x, hi.y, hi.z), Vec3::new(hi.x, hi.y, hi.z), Vec3::new(hi.x, hi.y, lo.z)]),
        (Vec3::NEG_Y, [Vec3::new(lo.x, lo.y, lo.z), Vec3::new(hi.x, lo.y, lo.z), Vec3::new(hi.x, lo.y, hi.z), Vec3::new(lo.x, lo.y, hi.z)]),
        (Vec3::X, [Vec3::new(hi.x, lo.y, lo.z), Vec3::new(hi.x, hi.y, lo.z), Vec3::new(hi.x, hi.y, hi.z), Vec3::new(hi.x, lo.y, hi.z)]),
        (Vec3::NEG_X, [Vec3::new(lo.x, lo.y, lo.z), Vec3::new(lo.x, lo.y, hi.z), Vec3::new(lo.x, hi.y, hi.z), Vec3::new(lo.x, hi.y, lo.z)]),
        (Vec3::Z, [Vec3::new(lo.x, lo.y, hi.z), Vec3::new(hi.x, lo.y, hi.z), Vec3::new(hi.x, hi.y, hi.z), Vec3::new(lo.x, hi.y, hi.z)]),
        (Vec3::NEG_Z, [Vec3::new(lo.x, lo.y, lo.z), Vec3::new(lo.x, hi.y, lo.z), Vec3::new(hi.x, hi.y, lo.z), Vec3::new(hi.x, lo.y, lo.z)]),
    ];
    for (normal, corners) in faces {
        let base = mesh.positions.len() as u16;
        for c in corners {
            mesh.positions.push(c.to_array());
            mesh.normals.push(normal.to_array());
            mesh.uvs.push([0.0, 0.0]);
            mesh.colors.push(if normal == Vec3::Z { front } else { [1.0, 1.0, 1.0, 1.0] });
            mesh.joints.push([0; 4]);
            mesh.weights.push([1.0, 0.0, 0.0, 0.0]);
        }
        mesh.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    mesh.material = MaterialDesc { base_color: color, vertex_color: true, lighting: Lighting::default(), ..MaterialDesc::default() };
    Model { source: ModelSource::Meshes { meshes: vec![mesh], textures: Vec::new(), joint_count: 1 }, parents: vec![None], groups: Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_closed_box_its_faces_outwards() {
        let model = box_model(Vec3::new(-1.0, 0.0, -2.0), Vec3::new(1.0, 1.0, 2.0), [1.0; 4], [1.0, 0.5, 0.5, 1.0]);
        let ModelSource::Meshes { meshes, .. } = &model.source else { panic!("meshes") };
        let m = &meshes[0];
        assert_eq!((m.positions.len(), m.indices.len()), (24, 36));
        for t in m.indices.chunks(3) {
            let [a, b, c] = [0, 1, 2].map(|i| Vec3::from_array(m.positions[t[i] as usize]));
            let n = Vec3::from_array(m.normals[t[0] as usize]);
            assert!((b - a).cross(c - a).dot(n) > 0.0, "counter-clockwise from outside");
        }
    }
}
