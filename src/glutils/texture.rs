use gl;
use image::{self, DynamicImage, GenericImage};
use std::os::raw::c_void;

// La structure Texture contient maintenant les dimensions de l'image.
#[derive(Default)]
pub struct Texture {
    pub id: u32,
    pub path: String,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    /// Crée une nouvelle texture OpenGL à partir d'un fichier image.
    pub fn new(path: &str) -> Self {
        let mut texture = Texture {
            id: 0,
            path: path.to_string(),
            width: 0,
            height: 0,
        };

        unsafe {
            // Générer un identifiant de texture
            gl::GenTextures(1, &mut texture.id);

            // Charger l'image depuis le disque avec la caisse `image`
            let img = image::open(&texture.path)
                .unwrap_or_else(|e| panic!("Échec du chargement de la texture à '{}': {}", texture.path, e));

            // Récupérer les dimensions et les stocker dans la structure
            let (width, height) = img.dimensions();
            texture.width = width;
            texture.height = height;

            // Convertir l'image au format RGBA8 pour une compatibilité maximale
            let data = match &img {
                DynamicImage::ImageRgba8(image) => image.to_vec(),
                _ => img.to_rgba().to_vec(),
            };

            // Lier la texture pour la configurer
            gl::BindTexture(gl::TEXTURE_2D, texture.id);

            // Envoyer les données de l'image à la VRAM de la carte graphique
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0, // Niveau de mipmap
                gl::RGBA as i32, // Format interne
                texture.width as i32,
                texture.height as i32,
                0, // Toujours 0 (pour la bordure)
                gl::RGBA, // Format des données source
                gl::UNSIGNED_BYTE, // Type des données source
                data.as_ptr() as *const c_void,
            );

            // Générer les mipmaps pour de meilleures performances et un meilleur rendu
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Configurer les options de la texture
            // WRAPPING: Comment la texture se répète si on la dessine sur une surface plus grande
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // FILTERING: Comment la texture est rendue quand elle est agrandie ou rétrécie
            // GL_LINEAR offre un rendu plus lisse que GL_NEAREST
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Délier la texture pour nettoyer l'état OpenGL
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        texture
    }

    /// Active une unité de texture spécifique (ex: TEXTURE0, TEXTURE1, ...).
    pub fn active(&self, unit: u32) {
        unsafe {
            // GL_TEXTURE0 + 0 = GL_TEXTURE0
            // GL_TEXTURE0 + 1 = GL_TEXTURE1
            gl::ActiveTexture(gl::TEXTURE0 + unit);
        }
    }

    /// Lie cette texture à l'unité de texture actuellement active.
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Délie la texture de l'unité de texture active (lie la texture 0 à la place).
    #[allow(dead_code)]
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}