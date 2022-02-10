#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use jni::JNIEnv;
    use jni::objects::{JString, JClass, JObject};
    use jni::sys::{jlong};

    use std::path::Path;
    use log::{Level, trace};

    extern crate android_logger;
    use android_logger::Config;

    #[no_mangle]
    pub unsafe extern fn Java_space_taran_arklib_ArkLib_provideIndexNative(
        env: JNIEnv,
        _: JClass,
        root_path: JString) -> jlong {

        android_logger::init_once(
            Config::default().with_min_level(Level::Trace));

        let root_path_str: String = env
            .get_string(root_path)
            .expect("Failed to parse root folder path")
            .into();
        let root_path_rs: &Path = Path::new(&root_path_str);
        trace!("Received path: {}", root_path_rs.display());

        arklib::provide_index(root_path_rs);
        return 123;
    }
}
