#![allow(non_upper_case_globals)]
extern crate glfw;


use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::{CStr, CString};

use cgmath::{Matrix4, vec3,  Rad, SquareMatrix, Matrix, Deg, perspective, Point3, Vector3};
use cgmath::prelude::*;
mod shader;
use glfw::Glfw;
use shader::Shader;
use image::GenericImage;

// settings
const SCR_WIDTH: u32 = 1080;
const SCR_HEIGHT: u32 = 720;

struct Camera{
    pub pos: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub sensitivity: f32,
    pub speed: f32,

}

struct Game {
    pub wireframe: bool,
    pub mix_value: f32,
    pub glfw_game: Glfw,
    pub move_x: f32,
    pub move_y: f32,
    pub move_z: f32,
    pub speed: f32,
    pub delta_time: f32,
}

#[allow(non_snake_case)]
pub fn main() {
    let mut game = Game {
        wireframe: false,
        mix_value: 0.2,
        glfw_game: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
        move_x: 0.0,
        move_y: 0.0,
        move_z: 0.0,
        speed: 3.0,
        delta_time: 0.0,
    };
    let mut camera = Camera {
        pos: Point3::new(0.0, 0.0, 0.0),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        yaw: -90.0,
        pitch: 0.0,
        last_x: SCR_WIDTH as f32 / 2.,
        last_y: SCR_HEIGHT as f32 / 2.,
        sensitivity: 0.04,
        speed: 2.0,
    };
    // glfw: initialize and configure
    // ------------------------------
    game.glfw_game.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    game.glfw_game.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    // glfw window creation
    // --------------------
    let (mut window, events) = game.glfw_game.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (our_shader, VAO, EBO, VBO, texture1, texture2) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        let our_shader = Shader::new(
            "src/shaders/vertex.glsl",
            "src/shaders/fragment.glsl"
        );

        gl::Enable(gl::DEPTH_TEST);
        
        // LINKING VERTEX ATTRIBUTES
       
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 180] = [
             -0.5, -0.5, -0.5,  0.0, 0.0,
              0.5, -0.5, -0.5,  1.0, 0.0,
              0.5,  0.5, -0.5,  1.0, 1.0,
              0.5,  0.5, -0.5,  1.0, 1.0,
             -0.5,  0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 0.0,

             -0.5, -0.5,  0.5,  0.0, 0.0,
              0.5, -0.5,  0.5,  1.0, 0.0,
              0.5,  0.5,  0.5,  1.0, 1.0,
              0.5,  0.5,  0.5,  1.0, 1.0,
             -0.5,  0.5,  0.5,  0.0, 1.0,
             -0.5, -0.5,  0.5,  0.0, 0.0,

             -0.5,  0.5,  0.5,  1.0, 0.0,
             -0.5,  0.5, -0.5,  1.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,
             -0.5, -0.5,  0.5,  0.0, 0.0,
             -0.5,  0.5,  0.5,  1.0, 0.0,

              0.5,  0.5,  0.5,  1.0, 0.0,
              0.5,  0.5, -0.5,  1.0, 1.0,
              0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5,  0.5,  0.0, 0.0,
              0.5,  0.5,  0.5,  1.0, 0.0,

             -0.5, -0.5, -0.5,  0.0, 1.0,
              0.5, -0.5, -0.5,  1.0, 1.0,
              0.5, -0.5,  0.5,  1.0, 0.0,
              0.5, -0.5,  0.5,  1.0, 0.0,
             -0.5, -0.5,  0.5,  0.0, 0.0,
             -0.5, -0.5, -0.5,  0.0, 1.0,

             -0.5,  0.5, -0.5,  0.0, 1.0,
              0.5,  0.5, -0.5,  1.0, 1.0,
              0.5,  0.5,  0.5,  1.0, 0.0,
              0.5,  0.5,  0.5,  1.0, 0.0,
             -0.5,  0.5,  0.5,  0.0, 0.0,
             -0.5,  0.5, -0.5,  0.0, 1.0
        ];
        // world space positions of our cubes
        
        let indices = [ // note that we start from 0!
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let mut VBO = 0;
        let mut VAO = 0;
        let mut EBO = 0;

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        // vertex arrays go first!
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as isize,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );
        
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        // gl::BufferData(
        //     gl::ELEMENT_ARRAY_BUFFER,
        //     (indices.len() * mem::size_of::<GLint>()) as isize,
        //     &indices[0] as *const i32 as *const c_void,
        //     gl::STATIC_DRAW
        // );
        // x, y, z, s, t, 
        // coords, texcoords
        let stride = 5 * mem::size_of::<GLfloat>() as GLint;
        // positions
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // colors
        // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>() as GLint) as *const c_void);
        // gl::EnableVertexAttribArray(1);
        // texture coords
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>() as GLint) as *const c_void);
        gl::EnableVertexAttribArray(1);
        // since we set the location = 0,
        // we enable the vertex attribute array at that location 
        // in this case 0

        // LOAD AND CREATE TEXTURE 
        //-------------------
        let mut texture1 = 0;
        gl::GenTextures(1, &mut texture1);
        // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        // * TEXTURE WRAPPING
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, gl::MIRRORED_REPEAT as GLint);

        // * TEXTURE FILTERING
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // * MIPMAP LEVELS
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // * Importing the texture from an image
        let img_1 = image::open(&Path::new("src/assets/brick_wall.jpg")).expect("couldnt find brick wall png");
        let img_1_data = img_1.raw_pixels();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0, 
            gl::RGB as GLint,
            img_1.width() as GLint,
            img_1.height() as GLint,
            0, 
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &img_1_data[0] as *const u8 as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        let mut texture2 = 0;
        gl::GenTextures(1, &mut texture2);
        // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

        // * TEXTURE WRAPPING
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, gl::MIRRORED_REPEAT as GLint);

        // * TEXTURE FILTERING
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        // * MIPMAP LEVELS
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        let img_2 = image::open(&Path::new("src/assets/awesomeface.png")).expect("couldnt find brick wall png");
        let img_2 = img_2.flipv();
        let img_2_data = img_2.raw_pixels();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0, 
            gl::RGB as GLint,
            img_2.width() as GLint,
            img_2.height() as GLint,
            0, 
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &img_2_data[0] as *const u8 as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);


        // tell opengl which texture unit we should use in our shaders
        our_shader.useProgram();
        our_shader.setInt(String::from("texture1"), 0);
        our_shader.setInt(String::from("texture2"), 1);
        (our_shader, VAO, EBO, VBO, texture1, texture2)
    };
    use cgmath::Vector3;
    let cube_positions: [Vector3<f32>; 10] = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5)
    ];

    
    let mut last_frame = 0.0;
    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events, &mut game, &mut camera);
        
        // render
        // ------
        unsafe {
            let current_frame = game.glfw_game.get_time() as GLfloat;
            game.delta_time = current_frame - last_frame;
            last_frame = current_frame;

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            our_shader.useProgram();
            //  draw our first triangle
            // gl::Uniform4f(vertex_color_location, 0.0, green_val, 0.0, 1.0);
            // to use both textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            our_shader.setFloat(String::from("mixValue"), game.mix_value);
            
            // let mut trans = cgmath::Matrix4::<f32>::identity();
            // trans = trans * Matrix4::<f32>::from_translation(vec3(game.move_x, game.move_y, 0.0));
            // trans = trans * Matrix4::<f32>::from_angle_z(Rad(game.glfw_game.get_time() as GLfloat * game.speed));
            // trans = trans * Matrix4::<f32>::from_angle_y(Rad(game.glfw_game.get_time() as GLfloat * -game.speed));


            let view_matrix = Matrix4::<f32>::look_at(
                camera.pos, 
                camera.pos + camera.front,
                camera.up,
            );
           
            let mut model = Matrix4::<f32>::identity();
            // moves the model on the floor 
            // x and z
            // model = model * Matrix4::<f32>::from_translation(vec3(game.move_x, game.move_y, 0.0));

            let mut view = Matrix4::<f32>::identity();
            view = view * Matrix4::<f32>::from_translation(vec3(0., 0., -3.0));
            // * better to use this! 
            // moves the model up and down left to right
            // x and y 
            // view = view * Matrix4::<f32>::from_translation(vec3(game.move_x, game.move_y, -3.0));

            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);


            let view_loc = gl::GetUniformLocation(our_shader.ID, CString::new("view").unwrap().as_c_str().as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_matrix.as_ptr());

            let projection_loc = gl::GetUniformLocation(our_shader.ID, CString::new("projection").unwrap().as_c_str().as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

            // let transform_location = gl::GetUniformLocation(our_shader.ID, CString::new("transform").unwrap().as_c_str().as_ptr());
            // gl::UniformMatrix4fv(transform_location, 1, gl::FALSE, trans.as_ptr());

            if game.wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
            else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
            gl::BindVertexArray(VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            for i in 0..cube_positions.len(){
                model = Matrix4::<f32>::identity();
                model = model * Matrix4::<f32>::from_translation(cube_positions[i]);
                model = model * Matrix4::<f32>::from_angle_x(Deg(20.0 * i as f32));
                model = model * Matrix4::<f32>::from_angle_y(Rad(game.glfw_game.get_time() as GLfloat));
                model = model * Matrix4::<f32>::from_angle_z(Rad(game.glfw_game.get_time() as GLfloat));
                // let model_loc = gl::GetUniformLocation(our_shader.ID, CString::new("model").unwrap().as_c_str().as_ptr());
                // gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
                our_shader.setMat4(String::from("model"), model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        game.glfw_game.poll_events();
    }
    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &VAO);
        gl::DeleteBuffers(1, &VBO);
        gl::DeleteBuffers(1, &EBO);
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(
    window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>, 
    game: &mut Game,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            glfw::WindowEvent::Key(Key::Tab, _, Action::Press, _) => game.wireframe = !game.wireframe,
            glfw::WindowEvent::Key(Key::Up, _, Action::Repeat, _) => game.mix_value += 0.008,
            glfw::WindowEvent::Key(Key::Down, _, Action::Repeat, _) => game.mix_value -= 0.008,

            // * mouse callback function
            glfw::WindowEvent::CursorPos(pos_x, pos_y) => {
                let (pos_x, pos_y) = (pos_x as f32, pos_y as f32);
                // get the offsets of positions
                let mut offset_x = pos_x - camera.last_x;
                // reversed because y is reversed
                let mut offset_y = camera.last_y - pos_y;  
                camera.last_x = pos_x;
                camera.last_y = pos_y;
                offset_x *= camera.sensitivity * 0.01;
                offset_y *= camera.sensitivity * 0.01;
                // add the offsets to our yaw and pitch values
                camera.yaw = camera.yaw + offset_x % 360.0;
                camera.pitch += offset_y;
                // assign some constraints so we cant look too high or too low
                if camera.pitch > 1.5{
                    camera.pitch = 1.5;
                }
                if camera.pitch < -1.5{
                    camera.pitch = -1.5;
                }
                // ? finally calculate the camera direction using some tringoneemytryrytytjskj :)
                let mut cam_dir = Vector3::<f32>::new(0.0, 0.0, 0.0);
                cam_dir.x = Rad(camera.yaw).cos() * Rad(camera.pitch).cos();
                cam_dir.y = Rad(camera.pitch).sin();
                cam_dir.z = Rad(camera.yaw).sin() * Rad(camera.pitch).cos();
                camera.front = cam_dir.normalize();

            }
            _ => {}
        }
    }

    if window.get_key(Key::W) == Action::Press {
        camera.pos += camera.speed * camera.front * game.delta_time;
    }
    if window.get_key(Key::S) == Action::Press {
        camera.pos -= camera.speed * camera.front * game.delta_time;
    }
    if window.get_key(Key::A) == Action::Press {
        camera.pos -= camera.front.cross(camera.up).normalize() * camera.speed * game.delta_time;
    }
    if window.get_key(Key::D) == Action::Press {
        camera.pos += camera.front.cross(camera.up).normalize() * camera.speed * game.delta_time;
    }
    if window.get_key(Key::Space) == Action::Press {
        camera.pos += camera.up * camera.speed * game.delta_time;
    }
    if window.get_key(Key::LeftControl) == Action::Press {
        camera.pos -= camera.up * camera.speed * game.delta_time;
    }
    if game.mix_value >= 1.0 {
        game.mix_value = 1.0;
    } else if game.mix_value <= 0.0{
        game.mix_value = 0.0;
    }
}
