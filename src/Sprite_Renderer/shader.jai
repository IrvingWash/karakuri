#scope_module

#import "Basic";
#import "GL";
#import "File";
#import "Math";

Shader :: struct {
    program_id: u32;
}

create_shader :: (vertex_src: string, fragment_src: string) -> Shader {
    c_vertex_src   := temp_c_string(vertex_src);
    c_fragment_src := temp_c_string(fragment_src);

    vertex := glCreateShader(GL_VERTEX_SHADER);
    defer glDeleteShader(vertex);
    glShaderSource(vertex, 1, *c_vertex_src, null);
    glCompileShader(vertex);
    check_compile_error(vertex);

    fragment := glCreateShader(GL_FRAGMENT_SHADER);
    defer glDeleteShader(fragment);
    glShaderSource(fragment, 1, *c_fragment_src, null);
    glCompileShader(fragment);
    check_compile_error(fragment);

    shader_program := glCreateProgram();
    glAttachShader(shader_program, vertex);
    glAttachShader(shader_program, fragment);
    glLinkProgram(shader_program);
    check_linking_error(shader_program);

    return .{ program_id = shader_program };
}

use_shader :: (shader: Shader) {
    glUseProgram(shader.program_id);
}

set_uniform :: (shader: Shader, name: string, values: ..$T)
#modify {
    return IsScalar(T) || T == Matrix4;
}
{
    c_name   := temp_c_string(name);
    location := glGetUniformLocation(shader.program_id, c_name);

    if location == -1 {
        return;
    }

    #if T == float32 {
        assert(values.count <= 4, "OpenGL's API allows to send up to 4 floats at once");

        if values.count == {
            case 1;
                glUniform1f(location, values[0]);
            case 2;
                glUniform2f(location, values[0], values[1]);
            case 3;
                glUniform3f(location, values[0], values[1], values[2]);
            case 4;
                glUniform4f(location, values[0], values[1], values[2], values[3]);
        }
    }

    #if T == s32 {
        assert(values.count <= 4, "OpenGL's API allows to send up to 4 ints at once");

        if values.count == {
            case 1;
                glUniform1i(location, values[0]);
            case 2;
                glUniform2i(location, values[0], values[1]);
            case 3;
                glUniform3i(location, values[0], values[1], values[2]);
            case 4;
                glUniform4i(location, values[0], values[1], values[2], values[3]);
        }
    }

    #if T == Matrix4 {
        assert(values.count <= 4, "OpenGL's API allows to send up to 1 matrix at once");

        glUniformMatrix4fv(
            location,
            1, // Matrix count
            GL_TRUE, // Should transpose
            xx *values[0]
        );
    }
}

destroy_shader :: (shader: Shader) {
    glDeleteProgram(shader.program_id);
}

#scope_file

INFO_LOG_BUFFER_SIZE :: 512;

check_compile_error :: (shader: u32) {
    ok: s32;

    glGetShaderiv(shader, GL_COMPILE_STATUS, *ok);
    if !ok {
        log_data: [INFO_LOG_BUFFER_SIZE]u8;

        glGetShaderInfoLog(shader, log_data.count, null, log_data.data);

        log_data_text := to_string(log_data.data);
        defer free(log_data_text);

        log_error("Failed to compile shader %\n", log_data_text);
    }
}

check_linking_error :: (program: u32) {
    ok: s32;

    glGetProgramiv(program, GL_LINK_STATUS, *ok);
    if !ok {
        log_data: [INFO_LOG_BUFFER_SIZE]u8;

        glGetProgramInfoLog(program, log_data.count, null, log_data.data);

        log_data_text := to_string(log_data.data);
        defer free(log_data_text);

        log_error(
            "Failed to link shader program: %\n",
            log_data_text,
        );
    }
}
