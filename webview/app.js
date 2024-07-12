window.onload = function() {
    // Get the WebGL rendering context
    var canvas = document.getElementById("webgl-canvas");
    var gl = canvas.getContext("webgl");

    if (!gl) {
        console.error("Unable to initialize WebGL. Your browser may not support it.");
        return;
    }

    // Vertex shader program source code
    var vsSource = `
        attribute vec4 aVertexPosition;
        varying highp vec2 pos;
        void main(void) {
            gl_Position = aVertexPosition;
            pos = gl_Position.xy;
        }
    `;

    // Fragment shader program source code
    var fsSource = `
        varying highp vec2 pos;
        void main(void) {
            gl_FragColor = vec4(0.5+0.5*pos, 1.0, 1.0);
        }
    `;

    // Compile shaders
    function compileShader(gl, source, type) {
        var shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);

        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            console.error('ERROR compiling shader:', gl.getShaderInfoLog(shader));
            gl.deleteShader(shader);
            return null;
        }

        return shader;
    }

    var vertexShader = compileShader(gl, vsSource, gl.VERTEX_SHADER);
    var fragmentShader = compileShader(gl, fsSource, gl.FRAGMENT_SHADER);

    // Link shaders into a program
    var shaderProgram = gl.createProgram();
    gl.attachShader(shaderProgram, vertexShader);
    gl.attachShader(shaderProgram, fragmentShader);
    gl.linkProgram(shaderProgram);

    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
        console.error('Unable to initialize the shader program:', gl.getProgramInfoLog(shaderProgram));
    }

    // Use the program
    gl.useProgram(shaderProgram);

    // Define vertex positions
    var vertices = new Float32Array([
        0.0,  1.0,
       -1.0, -1.0,
        1.0, -1.0,
    ]);
    var vertices = new Float32Array(1000);  // Adjust the number of vertices as needed
    for (var i = 0; i < vertices.length; i += 2) {
        var t = (i / vertices.length) * 4 * Math.PI;
        vertices[i] = t/10.0;
        vertices[i + 1] = Math.sin(t);
    }

    // Create a buffer and put the positions in it
    var vertexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

    // Bind buffer data to attribute
    var position = gl.getAttribLocation(shaderProgram, "aVertexPosition");
    gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(position);

    // Set clear color
    gl.clearColor(0.0, 0.0, 0.0, 1.0);

    // Clear the canvas and draw the triangle
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.LINE_STRIP, 0, vertices.length/2);
};