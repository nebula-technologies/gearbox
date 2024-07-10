#[macro_export]
macro_rules! tracer_err {
    () => {{
        use core::any::Any;
        use $crate::alloc::{
            boxed::Box,
            string::{String, ToString},
        };
        $crate::error::tracer::TracerError::new(
            String::new().type_id(),
            Box::new("".to_string()),
            $crate::error::tracer::ErrorTracerExtInfo::new(
                Some(line!()),
                Some(file!().to_string()),
                Some(module_path!().to_string()),
                None,
            ),
            None,
        )
    }};
    ($e:expr) => {{
        use core::any::Any;
        use $crate::alloc::{boxed::Box, string::ToString};
        let type_id = $e.type_id();
        $crate::error::tracer::TracerError::new(
            type_id,
            Box::new($e),
            $crate::error::tracer::ErrorTracerExtInfo::new(
                Some(line!()),
                Some(file!().to_string()),
                Some(module_path!().to_string()),
                None,
            ),
            None,
        )
    }};
}

#[macro_export]
macro_rules! tracer_dyn_err {
    () => {{
        use core::any::Any;
        use $crate::alloc::{
            boxed::Box,
            string::{String, ToString},
        };
        $crate::error::tracer::DynTracerError::new(
            Box::new("".to_string()),
            $crate::error::tracer::ErrorTracerExtInfo::new(
                Some(line!()),
                Some(file!().to_string()),
                Some(module_path!().to_string()),
                None,
            ),
            None,
        )
    }};
    ($e:expr) => {{
        use core::any::Any;
        use $crate::alloc::{boxed::Box, string::ToString};
        $crate::error::tracer::DynTracerError::new(
            Box::new($e),
            $crate::error::tracer::ErrorTracerExtInfo::new(
                Some(line!()),
                Some(file!().to_string()),
                Some(module_path!().to_string()),
                None,
            ),
            None,
        )
    }};
}
#[macro_export]
macro_rules! error_info {
    () => {
        $crate::error::tracer::ErrorTracerExtInfo::default()
            .with_file(file!())
            .with_line(line!())
            .with_subsystem(module_path!())
    };
}
