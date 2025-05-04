/// Settings for the camera.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::CameraSettings;
///
/// let camera_settings: CameraSettings = CameraSettings {
///        sharpness: "50".to_string(),//or any value you want to modify
///        ..Default::default()
///    };
/// let test_camera_settings: CameraSettings = CameraSettings {
///           contrast: "50".to_string(),
///           sharpness: "50".to_string(),
///           brightness: "60".to_string(),
///           saturation: "0".to_string(),
///           quality: "100".to_string(),
///           timeout: "3000".to_string(),
///           iso: "300".to_string(),
///           output: "~/raspicam.jpg".parse().unwrap(),
/// };
///
/// assert_eq!(camera_settings, test_camera_settings);
/// ```
///
#[derive(Debug, PartialEq)]
pub struct CameraSettings {
    pub contrast: String,
    pub sharpness: String,
    pub brightness: String,
    pub saturation: String,
    pub quality: String,
    pub timeout: String,
    pub iso: String,
    pub output: String,
}

impl Default for CameraSettings {
    /// Initialize CameraSettings with the default values
    ///
    /// # Example
    ///
    /// ```
    /// use raspicam::image::settings::CameraSettings;
    ///
    /// let camera_settings: CameraSettings = CameraSettings::default();
    /// let test_camera_settings: CameraSettings = CameraSettings {
    ///           contrast: "50".to_string(),
    ///           sharpness: "30".to_string(),
    ///           brightness: "60".to_string(),
    ///           saturation: "0".to_string(),
    ///           quality: "100".to_string(),
    ///           timeout: "20".to_string(),
    ///           iso: "300".to_string(),
    ///           output: "./out.jpg".parse().unwrap(),
    /// };
    ///
    /// assert_eq!(camera_settings, test_camera_settings);
    /// ```
    ///
    fn default() -> CameraSettings {
        //-brightness=0.3 --saturation=2
        CameraSettings {
            contrast: "1".to_string(),
            sharpness: "1".to_string(),
            brightness: "0.3".to_string(),
            saturation: "2".to_string(),
            quality: "93".to_string(),
            timeout: "10ms".to_string(),
            iso: "300".to_string(),
            output: "./out.jpg".parse().unwrap(),
        }
    }
}

/// Settings for the image.
///
/// # Example
///
/// ```
/// use raspicam::image::camera_operations::click_image;
/// use raspicam::image::settings::ImageSettings;
///
/// let image_settings: ImageSettings = ImageSettings {
///        width: "50".to_string(),//or any value you want to modify
///        ..Default::default()
///    };
/// let test_image_settings: ImageSettings = ImageSettings {
///         width: "50".to_string(),
///         height: "200".to_string(),
///         rotation: "180".to_string(),
///         horizontal_flip: "false".to_string(),
///         vertical_flip: "false".to_string(),
/// };
///
/// assert_eq!(image_settings, test_image_settings);
/// ```
///
#[derive(Debug, PartialEq)]
pub struct ImageSettings {
    pub width: String,
    pub height: String,
    pub rotation: String,
    pub horizontal_flip: String,
    pub vertical_flip: String,
}

impl Default for ImageSettings {
    /// Initialize ImageSettings with the default values
    ///
    /// # Example
    ///
    /// ```
    /// use raspicam::image::settings::ImageSettings;
    ///
    /// let image_settings: ImageSettings = ImageSettings::default();
    /// let test_image_settings: ImageSettings = ImageSettings {
    ///         width: "200".to_string(),
    ///         height: "200".to_string(),
    ///         rotation: "180".to_string(),
    ///         horizontal_flip: "false".to_string(),
    ///         vertical_flip: "false".to_string(),
    /// };
    ///
    /// assert_eq!(image_settings, test_image_settings);
    /// ```
    ///
    fn default() -> ImageSettings {
        ImageSettings {
            width: "0".to_string(),
            height: "0".to_string(),
            rotation: "180".to_string(),
            horizontal_flip: "false".to_string(),
            vertical_flip: "false".to_string(),
        }
    }
}
