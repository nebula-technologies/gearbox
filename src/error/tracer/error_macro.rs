#[macro_export]
macro_rules! Error {
    () => {
        $crate::error::tracer::ErrorTracerExtInfo::default()
            .with_file(file!())
            .with_line(line!())
            .with_subsystem(module_path!())
    };
}
