//let edge1 = triangle.vertices[1] - triangle.vertices[0];
//let edge2 = triangle.vertices[2] - triangle.vertices[0];
//let h = ray.direction.cross(edge2);
//let a = edge1.dot(h);

//if a.abs() < 0.0001 {
    //return None;
//}

//let f = 1.0/a;
//let s = ray.origin - triangle.vertices[0];
//let u = f * s.dot(h);

//if u < 0.0 || u > 1.0 {
    //return None;
//}

//let q = s.cross(edge1);
//let v = f * ray.direction.dot(q);

//if v < 0.0 || u + v > 1.0 {
    //return None;
//}

//let t = f * edge2.dot(q);

//if t > 0.0001 {
    //Some(RayHit {
        //ray: &ray,
        //distance: t,
        //object: *self,
    //})
//} else {
    //None
//}
