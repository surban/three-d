use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window = Window::new(WindowSettings {
        title: "PBR!".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(2.0, 2.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    Loader::load(
        &["examples/assets/gltf/DamagedHelmet.glb"],
        move |mut loaded| {
            let (cpu_meshes, cpu_materials) = loaded
                .gltf("examples/assets/gltf/DamagedHelmet.glb")
                .unwrap();
            let material = Material::new(&context, &cpu_materials[0]).unwrap();
            let mut model = Mesh::new(&context, &cpu_meshes[0]).unwrap();
            model.cull = CullType::Back;

            let plane_material = Material {
                albedo: vec4(0.5, 0.7, 0.3, 1.0),
                ..Default::default()
            };
            let plane = Mesh::new(
                &context,
                &CPUMesh {
                    positions: vec![
                        -10000.0, -1.3, 10000.0, 10000.0, -1.3, 10000.0, 0.0, -1.3, -10000.0,
                    ],
                    normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                    ..Default::default()
                },
            )
            .unwrap();

            let ambient_light = AmbientLight {
                color: Color::WHITE,
                intensity: 0.4,
            };
            let mut directional_light0 =
                DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap();
            let mut directional_light1 =
                DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap();
            let mut spot_light = SpotLight::new(
                &context,
                2.0,
                Color::WHITE,
                &vec3(0.0, 0.0, 0.0),
                &vec3(0.0, -1.0, 0.0),
                20.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();

            // main loop
            window
                .render_loop(move |mut frame_input| {
                    camera.set_viewport(frame_input.viewport).unwrap();
                    control
                        .handle_events(&mut camera, &mut frame_input.events)
                        .unwrap();

                    let time = 0.001 * frame_input.accumulated_time;
                    let c = time.cos() as f32;
                    let s = time.sin() as f32;
                    directional_light0.set_direction(&vec3(-1.0 - c, -1.0, 1.0 + s));
                    directional_light1.set_direction(&vec3(1.0 + c, -1.0, -1.0 - s));
                    spot_light.set_position(&vec3(3.0 + c, 5.0 + s, 3.0 - s));
                    spot_light.set_direction(&-vec3(3.0 + c, 5.0 + s, 3.0 - s));

                    // Draw
                    directional_light0
                        .generate_shadow_map(&vec3(0.0, 0.0, 0.0), 2.0, 20.0, 1024, 1024, &[&model])
                        .unwrap();
                    directional_light1
                        .generate_shadow_map(&vec3(0.0, 0.0, 0.0), 2.0, 20.0, 1024, 1024, &[&model])
                        .unwrap();
                    spot_light
                        .generate_shadow_map(15.0, 1024, &[&model])
                        .unwrap();
                    Screen::write(&context, ClearState::default(), || {
                        plane.render_with_lighting(
                            RenderStates::default(),
                            &camera,
                            &plane_material,
                            Some(&ambient_light),
                            &[&directional_light0, &directional_light1],
                            &[&spot_light],
                            &[],
                        )?;

                        model.render_with_lighting(
                            RenderStates::default(),
                            &camera,
                            &material,
                            Some(&ambient_light),
                            &[&directional_light0, &directional_light1],
                            &[&spot_light],
                            &[],
                        )?;
                        Ok(())
                    })
                    .unwrap();

                    if args.len() > 1 {
                        // To automatically generate screenshots of the examples, can safely be ignored.
                        FrameOutput {
                            screenshot: Some(args[1].clone().into()),
                            exit: true,
                            ..Default::default()
                        }
                    } else {
                        FrameOutput::default()
                    }
                })
                .unwrap();
        },
    );
}