use rapier3d::{math::Vector, prelude::*};

pub(crate) struct PhysicsSet {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    collision_notify: Vec<Box<dyn FnMut(CollisionEvent)>>,
    contact_notify: Vec<Box<dyn FnMut(ContactForceEvent)>>,
}

impl PhysicsSet {
    pub(crate) fn new(ground_width: f32) -> Self {
        let rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(ground_width, 0.1, ground_width).build();
        collider_set.insert(collider);

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();

        Self {
            rigid_body_set,
            collider_set,
            physics_pipeline,
            gravity,
            integration_parameters,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            collision_notify: vec![],
            contact_notify: vec![],
        }
    }

    pub(crate) fn new_body(&mut self, position: Vector<f32>) -> (RigidBodyHandle, ColliderHandle) {
        /* Create the bounding ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(position)
            .linear_damping(0.001)
            .build();
        let collider = ColliderBuilder::cuboid(13.06 * 0.5, 5.64 * 0.5, 19.43 * 0.5)
            .restitution(0.7)
            .friction(0.001)
            .active_events(ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS)
            .build();
        let body_handle = self.rigid_body_set.insert(rigid_body);
        let collider_handle =
            self.collider_set
                .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
        (body_handle, collider_handle)
    }

    pub(crate) fn register_collision(&mut self, f: impl FnMut(CollisionEvent) + 'static) {
        self.collision_notify.push(Box::new(f));
    }

    pub(crate) fn _register_contact(&mut self, f: impl FnMut(ContactForceEvent) + 'static) {
        self.contact_notify.push(Box::new(f));
    }

    pub(crate) fn step(&mut self) {
        let physics_hooks = ();
        // let event_handler = ();

        // Initialize the event collector.
        let (collision_send, collision_recv) = rapier3d::crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = rapier3d::crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        while let Ok(collision_event) = collision_recv.try_recv() {
            // Handle the collision event.
            // println!("Received collision event: {:?}", collision_event);
            for notify in &mut self.collision_notify {
                notify(collision_event);
            }
        }

        while let Ok(contact_force_event) = contact_force_recv.try_recv() {
            // Handle the contact force event.
            // println!("Received contact force event: {:?}", contact_force_event);
            for notify in &mut self.contact_notify {
                notify(contact_force_event);
            }
        }
    }
}
