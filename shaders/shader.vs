#version 330 core

// Attributs fournis par le VBO du SpriteCreator
layout (location = 0) in vec2 aPos;      // Position en 2D (x, y)
layout (location = 1) in vec2 aTexCoord; // Coordonnées de texture (u, v)

// Sortie vers le Fragment Shader
out vec2 TexCoords;

// Matrices de transformation
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    // On passe les coordonnées de texture au fragment shader
    TexCoords = aTexCoord;
    
    // On transforme la position 2D en une position 3D (z=0)
    // puis en coordonnées d'écran via les matrices.
    gl_Position = projection * view * model * vec4(aPos, 0.0, 1.0);
}