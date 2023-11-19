use three_d::{Camera, ColorMaterial, Context, CpuMaterial, Gm, Mesh, RenderTarget, Window};
use three_d_asset::{vec3, Mat4, Srgba, TriMesh, Vec3};

const THRUST_BAR_X: f32 = -3.0;
const THRUST_BAR_Y: f32 = -3.0;
const THRUST_BAR_WIDTH: f32 = 0.2;
const THRUST_BAR_HEIGHT: f32 = 1.0;
const RUDDER_BAR_X: f32 = 0.;
const RUDDER_BAR_Y: f32 = -3.0;
const RUDDER_BAR_WIDTH: f32 = 1.0;
const RUDDER_BAR_HEIGHT: f32 = 0.2;
const CONTACT_BAR_X: f32 = 3.0;
const CONTACT_BAR_Y: f32 = -3.0;
const CONTACT_BAR_WIDTH: f32 = 0.2;
const CONTACT_BAR_HEIGHT: f32 = 0.2;

pub(crate) struct Ui {
    camera: Camera,
    // ui_grid_obj: Box<dyn Object>,
    thrust_bar_back: Gm<Mesh, ColorMaterial>,
    thrust_bar: Gm<Mesh, ColorMaterial>,
    rudder_bar_back: Gm<Mesh, ColorMaterial>,
    rudder_bar: Gm<Mesh, ColorMaterial>,
    contact_back: Gm<Mesh, ColorMaterial>,
    contact: Gm<Mesh, ColorMaterial>,
    has_contact: bool,
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
            Mat4::from_translation(Vec3::new(THRUST_BAR_X, THRUST_BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(THRUST_BAR_WIDTH, THRUST_BAR_HEIGHT, 1.),
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
            Mat4::from_translation(Vec3::new(THRUST_BAR_X, THRUST_BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(THRUST_BAR_WIDTH, THRUST_BAR_HEIGHT, 1.),
        );

        let mut rudder_bar_back = Gm::new(
            Mesh::new(&context, &bar),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(0, 0, 0, 191),
                    ..Default::default()
                },
            ),
        );
        rudder_bar_back.set_transformation(
            Mat4::from_translation(Vec3::new(RUDDER_BAR_X, RUDDER_BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(RUDDER_BAR_WIDTH, RUDDER_BAR_HEIGHT, 1.),
        );

        let mut rudder_bar = Gm::new(
            Mesh::new(&context, &bar),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(255, 255, 0, 255),
                    ..Default::default()
                },
            ),
        );
        rudder_bar.set_transformation(
            Mat4::from_translation(Vec3::new(RUDDER_BAR_X, RUDDER_BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(RUDDER_BAR_WIDTH, RUDDER_BAR_HEIGHT, 1.),
        );

        let mut contact_back = Gm::new(
            Mesh::new(&context, &bar),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(0, 0, 0, 191),
                    ..Default::default()
                },
            ),
        );
        contact_back.set_transformation(
            Mat4::from_translation(Vec3::new(CONTACT_BAR_X, CONTACT_BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(CONTACT_BAR_WIDTH, CONTACT_BAR_HEIGHT, 1.),
        );

        let mut contact = Gm::new(
            Mesh::new(&context, &bar),
            ColorMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(255, 255, 0, 255),
                    ..Default::default()
                },
            ),
        );
        contact.set_transformation(
            Mat4::from_translation(Vec3::new(CONTACT_BAR_X, CONTACT_BAR_Y, 0.))
                * Mat4::from_nonuniform_scale(CONTACT_BAR_WIDTH, CONTACT_BAR_HEIGHT, 1.),
        );

        Self {
            camera,
            // ui_grid_obj: Box::new(ui_grid_obj),
            thrust_bar_back,
            thrust_bar,
            rudder_bar_back,
            rudder_bar,
            contact_back,
            contact,
            has_contact: false,
        }
    }

    pub(crate) fn render(&self, render: &RenderTarget) {
        let mut objects = vec![
            // self.ui_grid_obj.as_ref(),
            &self.thrust_bar_back,
            &self.thrust_bar,
            &self.rudder_bar_back,
            &self.rudder_bar,
            &self.contact_back,
        ];
        if self.has_contact {
            objects.push(&self.contact);
        }
        render.render(&self.camera, &objects, &[]);
    }

    pub(crate) fn update_thrust(&mut self, thrust: f32) {
        self.thrust_bar.set_transformation(
            Mat4::from_translation(Vec3::new(
                THRUST_BAR_X,
                THRUST_BAR_Y + THRUST_BAR_HEIGHT * (thrust - 1.),
                0.,
            )) * Mat4::from_nonuniform_scale(THRUST_BAR_WIDTH, THRUST_BAR_HEIGHT * thrust, 1.),
        );
    }

    pub(crate) fn update_rudder(&mut self, rudder: f32) {
        self.rudder_bar.set_transformation(
            Mat4::from_translation(Vec3::new(
                RUDDER_BAR_X + RUDDER_BAR_WIDTH * -rudder * 0.5,
                RUDDER_BAR_Y,
                0.,
            )) * Mat4::from_nonuniform_scale(
                RUDDER_BAR_WIDTH * -rudder * 0.5,
                RUDDER_BAR_HEIGHT,
                1.,
            ),
        );
    }

    pub(crate) fn update_has_contact(&mut self, v: bool) {
        self.has_contact = v;
    }
}
