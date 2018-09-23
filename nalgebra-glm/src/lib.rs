/*! # nalgebra-glm − nalgebra in _easy mode_
    **nalgebra-glm** is a GLM-like interface for the **nalgebra** general-purpose linear algebra library.
    [GLM](https://glm.g-truc.net) itself is a popular C++ linear algebra library essentially targeting computer graphics. Therefore
    **nalgebra-glm** draws inspiration from GLM to define a nice and easy-to-use API for simple graphics application.

    All the types of **nalgebra-glm** are aliases of types from **nalgebra**. Therefore there is a complete and
    seamless inter-operability between both.

    ## Getting started
    First of all, you should start by taking a look at the official [GLM API documentation](http://glm.g-truc.net/0.9.9/api/index.html)
    since **nalgebra-glm** implements a large subset of it. To use **nalgebra-glm** to your project, you
    should add it as a dependency to your `Crates.toml`:

    ```toml
    [dependencies]
    nalgebra-glm = "0.1"
    ```

    Then, you should add an `extern crate` statement to your `lib.rs` or `main.rs` file. It is **strongly
    recommended** to add a crate alias to `glm` as well so that you will be able to call functions of
    **nalgebra-glm** using the module prefix `glm::`. For example you will write `glm::rotate(...)` instead
    of the more verbose `nalgebra_glm::rotate(...)`:

    ```rust
    extern crate nalgebra_glm as glm;
    ```

    ## Features overview
    **nalgebra-glm** supports most linear-algebra related features of the C++ GLM library. Mathematically
    speaking, it supports all the common transformations like rotations, translations, scaling, shearing,
    and projections but operating in homogeneous coordinates. This means all the 2D transformations are
    expressed as 3x3 matrices, and all the 3D transformations as 4x4 matrices. This is less computationally-efficient
    and memory-efficient than nalgebra's [transformation types](https://www.nalgebra.org/points_and_transformations/#transformations),
    but this has the benefit of being simpler to use.
    ### Main differences compared to GLM
    While **nalgebra-glm** follows the feature line of the C++ GLM library, quite a few differences
    remain and they are mostly syntactic. The main ones are:
    * All function names use `snake_case`, which is the Rust convention.
    * All type names use `CamelCase`, which is the Rust convention.
    * All function arguments, except for scalars, are all passed by-reference.
    * Some feature are not yet implemented and should be added in the future. In particular, no packing
    functions are available.
    * A few features are not implemented and will never be. This includes functions related to color
    spaces, and closest points computations. Other crates should be used for those. For example, closest
    points computation can be handled by the [ncollide](https://ncollide.org) project.

    In addition, because Rust does not allows function overloading, all functions must be given a unique name.
    Here are a few rules chosen arbitrarily for **nalgebra-glm**:
    * Functions operating in 2d will usually end with the `2d` suffix, e.g., `glm::rotade2d` is for 2D while `glm::rotate` is for 3D.
    * Functions operating on vector will often end with the `_vec` suffix, possibly followed by the dimension of vector, e.g., `glm::rotate_vec2`.
    * Every function related to quaternions start with the `quat_` prefix, e.g., `glm::quat_dot(q1, q2)`.
    * All the conversion functions have unique names as described [bellow](#conversions).
    ### Vector and matrix construction
    Vectors, matrices, and quaternions can be constructed using several approaches:
    * Using functions with the same name as their type in lower-case. For example `glm::vec3(x, y, z)` will create a 3D vector.
    * Using the `::new` constructor. For example `Vec3::new(x, y, z)` will create a 3D vector.
    * Using the functions prefixed by `make_` to build a vector a matrix from a slice. For example `glm::make_vec3(&[x, y, z])` will create a 3D vector.
    Keep in mind that constructing a matrix using this type of funcitons require its components to be arrange in column-major order on the slice.
    * Using a geometric construction function. For example `glm::rotation(angle, axis)` will build a 4x4 homogeneous rotation matrix from an angle (in radians) and an axis.
    * Using swizzling and conversions as described in the next sections.
    ### Swizzling
    Vector swizzling is a native feature of **nalgebra** itself. Therefore, you can use it with all
    the vectors of **nalgebra-glm** as well. Swizzling is supported as methods and works only up to
    dimension 3, i.e., you can only refer to the components `x`, `y` and `z` and can only create a
    2D or 3D vector using this technique. Here is some examples, assuming `v` is a vector with float
    components here:
    * `v.xx()` is equivalent to `glm::vec2(v.x, v.x)` and to `Vec2::new(v.x, v.x)`.
    * `v.zx()` is equivalent to `glm::vec2(v.z, v.x)` and to `Vec2::new(v.z, v.x)`.
    * `v.yxz()` is equivalent to `glm::vec3(v.y, v.x, v.z)` and to `Vec3::new(v.y, v.x, v.z)`.
    * `v.zzy()` is equivalent to `glm::vec3(v.z, v.z, v.y)` and to `Vec3::new(v.z, v.z, v.y)`.

    Any combination of two or three components picked among `x`, `y`, and `z` will work.
    ### Conversions
    It is often useful to convert one algebraic type to another. There are two main approaches for converting
    between types in `nalgebra-glm`:
    * Using function with the form `type1_to_type2` in order to convert an instance of `type1` into an instance of `type2`.
    For example `glm::mat3_to_mat4(m)` will convert the 3x3 matrix `m` to a 4x4 matrix by appending one column on the right
    and one row on the left. Those now row and columns are filled with 0 except for the diagonal element which is set to 1.
    * Using one of the `convert`, `try_convert`, or `convert_unchecked` functions.
    These functions are directly re-exported from nalgebra and are extremely versatile:
        1. The `convert` function can convert any type (especially geometric types from nalgebra like `Isometry3`) into another algebraic type which equivalent but more general. For example,
    `let sim: Similarity3<_> = na::convert(isometry)` will convert an `Isometry3` into a `Similarity3`.
    In addition, `let mat: Mat4 = glm::convert(isometry)` will convert an `Isometry3` to a 4x4 matrix. This will also convert the scalar types,
    therefore: `let mat: DMat4 = glm::convert(m)` where `m: Mat4` will work. However, conversion will not work the other way round: you
    can't convert a `Matrix4` to an `Isometry3` using `glm::convert` because that could cause unexpected results if the matrix does
    not complies to the requirements of the isometry.
        2. If you need this kind of conversions anyway, you can use `try_convert` which will test if the object being converted complies with the algebraic requirements of the target type.
        This will return `None` if the requirements are not satisfied.
        3. The `convert_unchecked` will ignore those tests and always perform the conversion, even if that breaks the invariants of the target type.
        This must be used with care! This is actually the recommended method to convert between homogeneous transformations generated by `nalgebra-glm` and
        specific transformation types from **nalgebra** like `Isometry3`. Just be careful you know your conversions make sense.

    ### Should I use nalgebra or nalgebra-glm?
    Well that depends on your tastes and your background. **nalgebra** is more powerful overall since it allows stronger typing,
    and goes much further than simple computer graphics math. However, has a bit of a learning curve for
    those not used to the abstract mathematical concepts for transformations. **nalgebra-glm** however
    have more straightforward functions and benefit from the various tutorials existing on the internet
    for the original C++ GLM library.

    Overall, if you are already used to the C++ GLM library, or to working with homogeneous coordinates (like 4D
    matrices for 3D transformations), then you will have more success with **nalgebra-glm**. If on the other
    hand you prefer more rigorous treatments of transformations, with type-level restrictions, then go for **nalgebra**.
    If you need dynamically-sized matrices, you should go for **nalgebra** as well.

    Keep in mind that **nalgebra-glm** is just a different API for **nalgebra**. So you can very well use both
    and benefit from both their advantages: use **nalgebra-glm** when mathematical rigor is not that important,
    and **nalgebra** itself when you need more expressive types, and more powerful linear algebra operations like
    matrix factorizations and slicing. Just remember that all the **nalgebra-glm** types are just aliases to **nalgebra** types,
    and keep in mind it is possible to convert, e.g., an `Isometry3` to a `Mat4` and vice-versa (see the [conversions section](#conversions)).
 */

extern crate num_traits as num;
#[macro_use]
extern crate approx;
extern crate alga;
extern crate nalgebra as na;

pub use aliases::*;
pub use constructors::*;
pub use common::{abs, ceil, clamp, clamp_scalar, clamp_vec, float_bits_to_int, float_bits_to_int_vec, float_bits_to_uint, float_bits_to_uint_vec, floor, fract, int_bits_to_float, int_bits_to_float_vec, mix, modf, modf_vec, round, sign, smoothstep, step, step_scalar, step_vec, trunc, uint_bits_to_float, uint_bits_to_float_scalar};
pub use geometric::{reflect_vec, cross, distance, dot, faceforward, length, magnitude, normalize, refract_vec};
pub use matrix::{transpose, determinant, inverse, matrix_comp_mult, outer_product};
pub use traits::{Dimension, Number, Alloc};
pub use trigonometric::{acos, acosh, asin, asinh, atan, atan2, atanh, cos, cosh, degrees, radians, sin, sinh, tan, tanh};
pub use vector_relational::{all, any, equal, greater_than, greater_than_equal, less_than, less_than_equal, not, not_equal};
pub use exponential::{exp, exp2, inversesqrt, log, log2, pow, sqrt};

pub use gtx::{
    comp_add, comp_max, comp_min, comp_mul,
    cross2d,
    left_handed, right_handed,
    matrix_cross, matrix_cross3,
    diagonal2x2, diagonal2x3, diagonal2x4, diagonal3x2, diagonal3x3, diagonal3x4, diagonal4x2, diagonal4x3, diagonal4x4,
    distance2, l1_distance, l1_norm, l2_distance, l2_norm, length2, magnitude2,
    triangle_normal,
    fast_normalize_dot, normalize_dot,
    quat_rotate_normalized_axis, rotate_normalized_axis,
    orientation, rotate_vec2, rotate_vec3, rotate_vec4, rotate_x_vec4, rotate_x_vec3, rotate_y_vec4, rotate_y_vec3, rotate_z_vec4, rotate_z_vec3, slerp,
    rotation, scaling, translation, rotation2d, scaling2d, translation2d,
    proj, proj2d, reflect, reflect2d, scale_bias, scale_bias_matrix, shear2d_x, shear_x, shear_y, shear2d_y, shear_z,
    rotate2d, scale2d, translate2d,
    angle,
    are_collinear, are_collinear2d, are_orthogonal, is_comp_null, is_normalized, is_null,
    quat_to_mat3, quat_rotate_vec, quat_cross_vec, mat3_to_quat, quat_extract_real_component, quat_fast_mix, quat_inv_cross_vec, quat_length2, quat_magnitude2, quat_identity, quat_rotate_vec3, quat_rotation, quat_short_mix, quat_to_mat4, to_quat
};
pub use gtc::{
    e, two_pi, euler, four_over_pi, golden_ratio, half_pi, ln_ln_two, ln_ten, ln_two, one, one_over_pi, one_over_root_two, one_over_two_pi, quarter_pi, root_five, root_half_pi, root_ln_four, root_pi, root_three, root_two, root_two_pi, third, three_over_two_pi, two_over_pi, two_over_root_pi, two_thirds, zero,
    column, row, set_column, set_row,
    affine_inverse, inverse_transpose,
    make_mat2, make_mat2x2, make_mat2x3, make_mat2x4, make_mat3, make_mat3x2, make_mat3x3, make_mat3x4, make_mat4, make_mat4x2, make_mat4x3, make_mat4x4, make_quat, make_vec1, make_vec2, make_vec3, make_vec4, value_ptr, value_ptr_mut, vec1_to_vec2, vec1_to_vec3, vec1_to_vec4, vec2_to_vec1, vec2_to_vec2, vec2_to_vec3, vec2_to_vec4, vec3_to_vec1, vec3_to_vec2, vec3_to_vec3, vec3_to_vec4, vec4_to_vec1, vec4_to_vec2, vec4_to_vec3, vec4_to_vec4, mat2_to_mat3, mat2_to_mat4, mat3_to_mat2, mat3_to_mat4, mat4_to_mat2, mat4_to_mat3,
    quat_cast, quat_euler_angles, quat_greater_than, quat_greater_than_equal, quat_less_than, quat_less_than_equal, quat_look_at, quat_look_at_lh, quat_look_at_rh, quat_pitch, quat_roll, quat_yaw
};
pub use ext::{
    ortho, perspective,
    pick_matrix, project, project_no, project_zo, unproject, unproject_no, unproject_zo,
    equal_columns, equal_columns_eps, equal_columns_eps_vec, not_equal_columns, not_equal_columns_eps, not_equal_columns_eps_vec,
    identity, look_at, look_at_lh, rotate, scale, look_at_rh, translate, rotate_x, rotate_y, rotate_z,
    max3_scalar, max4_scalar, min3_scalar, min4_scalar,
    epsilon, pi,
    max, max2, max3, max4, min, min2, min3, min4,
    equal_eps, equal_eps_vec, not_equal_eps, not_equal_eps_vec,
    quat_conjugate, quat_inverse, quat_lerp, quat_slerp,
    quat_cross, quat_dot, quat_length, quat_magnitude, quat_normalize,
    quat_equal, quat_equal_eps, quat_not_equal, quat_not_equal_eps,
    quat_exp, quat_log, quat_pow, quat_rotate,
    quat_angle, quat_angle_axis, quat_axis
};

pub use na::{convert, convert_ref, convert_unchecked, convert_ref_unchecked, try_convert, try_convert_ref};

mod aliases;
mod constructors;
mod common;
mod matrix;
mod geometric;
mod traits;
mod trigonometric;
mod vector_relational;
mod exponential;
//mod integer;
//mod packing;

mod ext;
mod gtc;
mod gtx;