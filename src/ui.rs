use three_d::{Camera, ColorMaterial, Context, CpuMaterial, Gm, Mesh, RenderTarget, Window};
use three_d_asset::{vec3, Mat4, Srgba, TriMesh, Vec3};

const BAR_X: f32 = -3.0;
const BAR_Y: f32 = -3.0;
const BAR_WIDTH: f32 = 0.2;
const BAR_HEIGHT: f32 = 1.0;

pub(crate) struct Ui {
    camera: Camera,
    // ui_grid_obj: Box<dyn Object>,
    thrust_bar_back: Gm<Mesh, ColorMaterial>,
    thrust_bar: Gm<Mesh, ColorMaterial>,
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

        // let ui_grid = grid_mesh(10, 10, 0.5, 0.01);
        // let mut ui_grid_obj = Gm::new(
        //     Mesh::new(&context, &ui_grid),
        //     ColorMaterial::new(
        //         &context,
        //         &CpuMaterial {
        //             albedo: Srgba {
        //                 r: 0,
        //                 g: 255,
        //                 b: 0,
        //                 a: 255,
        //             },
        //             ..Default::default()
        //         },
        //     ),
        // );
        // ui_grid_obj.set_transformation(Mat4::from_angle_x(Deg(90.)));

        let bar = TriMesh::square();
        let mut thrust_bar_back = Gm::new(
            Mesh::new(&context, &bar),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(0, 0, 0, 191),
                    ..Default::default()
                },
            ),
        );
        thrust_bar_back.set_transformation(
            Mat4::from_translation(Vec3::new(BAR_X, BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(BAR_WIDTH, BAR_HEIGHT, 1.),
        );

        let mut thrust_bar = Gm::new(
            Mesh::new(&context, &bar),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(255, 0, 0, 255),
                    ..Default::default()
                },
            ),
        );
        thrust_bar.set_transformation(
            Mat4::from_translation(Vec3::new(BAR_X, BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(BAR_WIDTH, BAR_HEIGHT, 1.),
        );

        Self {
            camera,
            // ui_grid_obj: Box::new(ui_grid_obj),
            thrust_bar_back,
            thrust_bar,
        }
    }

    pub(crate) fn render(&self, render: &RenderTarget) {
        render.render(
            &self.camera,
            &[
                // self.ui_grid_obj.as_ref(),
                &self.thrust_bar_back,
                &self.thrust_bar,
            ],
            &[],
        );
    }

    pub(crate) fn update_thrust(&mut self, thrust: f32) {
        self.thrust_bar.set_transformation(
            Mat4::from_translation(Vec3::new(BAR_X, BAR_Y + BAR_HEIGHT * (thrust - 1.), 0.))
                * Mat4::from_nonuniform_scale(BAR_WIDTH, BAR_HEIGHT * thrust, 1.),
        );
    }
}
