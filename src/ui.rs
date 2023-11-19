use three_d::{Camera, ColorMaterial, Context, CpuMaterial, Gm, Mesh, Object, Window};
use three_d_asset::{vec3, Deg, Mat4, Srgba};

use crate::grid::grid_mesh;

pub(crate) struct Ui {
    pub camera: Camera,
    pub ui_grid_obj: Box<dyn Object>,
}

impl Ui {
    pub(crate) fn new(window: &Window, context: &Context) -> Self {
        let camera = Camera::new_orthographic(
            window.viewport(),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            10.0,
            0.1,
            1000.0,
        );

        let ui_grid = grid_mesh(10, 10, 0.5, 0.01);
        let mut ui_grid_obj = Gm::new(
            Mesh::new(&context, &ui_grid),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    },
                    ..Default::default()
                },
            ),
        );
        ui_grid_obj.set_transformation(Mat4::from_angle_x(Deg(90.)));
        Self {
            camera,
            ui_grid_obj: Box::new(ui_grid_obj),
        }
    }
}
